// This module handles direct memory region claimation
use core::marker::PhantomData;
use core::slice;
use lazy_static::lazy_static;
use spin::once::Once;
use spin::Mutex;

pub type GlobalMemoryMapper = TableMemoryMapper;
static GLOBAL_MEMORY_MAPPER: Once<GlobalMemoryMapper> = Once::new();

// WARNING: region must be usable or it may lead to undefined behaviour
pub unsafe fn set_global(region: (usize, usize)) {
    GLOBAL_MEMORY_MAPPER.call_once(|| GlobalMemoryMapper::manage(region));
}

pub fn claim_global(region: (usize, usize)) -> Result<Ptr<'static, [u8]>, MemoryMapperError> {
    GLOBAL_MEMORY_MAPPER
        .get()
        .expect("GLOBAL_MEMORY_MAPPER not setup!")
        .claim(region)
}

// TODO: free region when this drops
pub struct Ptr<'a, T: ?Sized> {
    raw: *mut T,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T: ?Sized> Ptr<'a, T> {
    fn new(x: *mut T) -> Self {
        Self {
            raw: x,
            _phantom: PhantomData,
        }
    }
}

pub trait MemoryMapper {
    // SAFETY! region must adhere to the following:
    // - must have read & write priviliges for ring0
    // - must not hold any other already used structures
    // - must not be managed by another MemoryMapper
    unsafe fn manage(region: (usize, usize)) -> Self;
    // in case the region is occupied, returns Err(occupant region)
    fn claim(&self, region: (usize, usize)) -> Result<Ptr<[u8]>, MemoryMapperError>;
    // returns true on success, and false if the region could not be found
    unsafe fn free(&self, region: (usize, usize)) -> bool;
}

pub enum MemoryMapperError {
    AlreadyOccupiedBy((usize, usize)), // contains the occupant region
    OutOfBound((usize, usize)),        // contains the valid Mapper region
}

// ========== Implementations

// TABLE MEMORY MAPPER
// Simple implementation of MemoryMapper that uses a statically sized array(table) to store entries.
// WARNING: will panic if table gets full
// WARNING TODO INVESTIGATE: Sometimes randomly triple faults, lowering TABLE_SIZE fixes the issue
const TABLE_SIZE: usize = 400; // max amount of entries
pub struct TableMemoryMapper {
    start: usize,
    end: usize,
    table: Mutex<[Option<(usize, usize)>; TABLE_SIZE]>,
}

impl MemoryMapper for TableMemoryMapper {
    unsafe fn manage(region: (usize, usize)) -> Self {
        Self {
            start: region.0,
            end: region.1,
            table: Mutex::new([None; TABLE_SIZE]),
        }
    }

    fn claim(&self, region: (usize, usize)) -> Result<Ptr<[u8]>, MemoryMapperError> {
        let (start, end) = region;
        // bound check the request
        if start < self.start || end > self.end {
            return Err(MemoryMapperError::OutOfBound((self.start, self.end)));
        }

        // loop through all the entries in the table
        let mut table = self.table.lock();
        for slot in table.iter() {
            // slot may be None and not hold an entry
            if let Some((first, last)) = *slot {
                // cover all possible intersections of regions
                if (first <= start && last >= start) || (first >= start && first <= end) {
                    return Err(MemoryMapperError::AlreadyOccupiedBy((first, last)));
                }
            }
        }

        // get first empty slot to store our entry
        let mut i = 0;
        while let Some(_) = table[i] {
            i += 1
        }
        // panic if no empty slot has been found
        if i >= TABLE_SIZE {
            panic!("Maximum number of MemoryMapper entries reached!");
        }

        table[i] = Some(region);
        Ok(Ptr::new(unsafe {
            slice::from_raw_parts_mut(start as *mut u8, end - start) as *mut [u8]
        }))
    }

    // WARNING: calling free() on a region that is still used may lead to undefind behaviour
    // returns true -> on success, false -> the target region has not been found
    unsafe fn free(&self, region: (usize, usize)) -> bool {
        let mut table = self.table.lock();
        for (i, taken) in table.iter().enumerate() {
            if taken == &Some(region) {
                table[i] = None;
                return true;
            }
        }
        false
    }
}
