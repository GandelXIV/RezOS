// This module handles direct memory region claimation
use core::marker::PhantomData;
use core::slice;
use lazy_static::lazy_static;
use spin::Mutex;

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

trait MemoryMapper {
    // SAFETY! region must adhere to the following:
    // - must have read & write priviliges for ring0
    // - must not hold any other already used structures
    // - must not be managed by another MemoryMapper
    unsafe fn manage(region: (usize, usize)) -> Self;
    // in case the region is occupied, returns Err(occupant region)
    fn claim(&self, region: (usize, usize)) -> Result<Ptr<[u8]>, MemoryMapperError>;
    // returns true on success, and false if the region could not be found
    fn free(&self, region: (usize, usize)) -> bool;
}

pub enum MemoryMapperError {
    AlreadyOccupiedBy((usize, usize)), // contains the occupant region
    InvalidRegion((usize, usize)),     // contains the whole Mapper region
}

// TABLE MEMORY MAPPER
// Simple implementation of MemoryMapper that uses a statically sized array(table) to store entries.
// WARNING: will panic if table gets full
const TABLE_SIZE: usize = 1024; // max amount of entries
struct TableMemoryMapper {
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
        if start < self.start || end > self.end {
            return Err(MemoryMapperError::InvalidRegion((self.start, self.end)));
        }

        let mut table = self.table.lock();
        for taken in table.iter() {
            if let Some((first, last)) = *taken {
                if (first <= start && last >= start) || (first >= start && first <= end) {
                    return Err(MemoryMapperError::AlreadyOccupiedBy((first, last)));
                }
            }
        }

        let mut i = 0;
        while let Some(_) = table[i] {
            i += 1
        }
        if i >= TABLE_SIZE {
            panic!("Maximum number of MemoryMapper entries reached!");
        }
        table[i] = Some(region);
        Ok(Ptr::new(unsafe {
            slice::from_raw_parts_mut(start as *mut u8, end - start) as *mut [u8]
        }))
    }

    fn free(&self, region: (usize, usize)) -> bool {
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
