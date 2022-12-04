// This module handles direct memory region claimation
use crate::log;
use core::mem;
use spin::once::Once;
use spin::Mutex;

pub type GlobalMemoryMapper = TableMemoryMapper;
pub static GLOBAL_MEMORY_MAPPER: Once<GlobalMemoryMapper> = Once::new();

// WARNING: region must be usable or it may lead to undefined behaviour
pub unsafe fn set_global(region: (usize, usize)) {
    GLOBAL_MEMORY_MAPPER.call_once(|| GlobalMemoryMapper::manage(region));
}

pub fn claim_global(region: (usize, usize)) -> Result<MapArea, MemoryMapperError> {
    GLOBAL_MEMORY_MAPPER
        .get()
        .expect("GLOBAL_MEMORY_MAPPER not setup!")
        .claim(region)
}

type MapItem = (usize, usize);

// I -> iterator returned when memory map requested
pub trait MemoryMapper<I: Iterator<Item = MapItem>> {
    // SAFETY! region must adhere to the following:
    // - must have read & write priviliges for ring0
    // - must not hold any other already used structures
    // - must not be externally managed by another MemoryMapper
    unsafe fn manage(region: (usize, usize)) -> Self;
    // in case the region is occupied, returns Err(occupant region)
    fn claim(&self, region: (usize, usize)) -> Result<MapArea, MemoryMapperError>;
    // WARNING: free() should panic if area was not found, because it implies one of the following:
    //  1. the area was never claimed and the structure used has been invalid the whole time -> UB
    //  2. the area was force_freed() which means it may have been claimed by some other entity in
    //     the mean time -> UB
    fn free(&self, area: MapArea);
    // WARNING: if the region is part of a used living Area, this may lead to UB.
    // Use ONLY if you are sure that the owning Area is dead/not used.
    unsafe fn force_free(&self, region: (usize, usize)) {
        self.free(MapArea::new(region));
    }
    // Iterate through claimed regions
    fn iter(&self) -> I;
    // Iterator through unclaimed regions
    fn gaps(&self) -> MapGaps<I> {
        MapGaps {
            iter: self.iter(),
            last: self.dimensions().0,
        }
    }
    // getters
    fn dimensions(&self) -> MapItem;
}

pub struct MapGaps<I: Iterator<Item = MapItem>> {
    iter: I,
    last: usize,
}

impl<I> Iterator for MapGaps<I>
where
    I: Iterator<Item = MapItem>,
{
    type Item = MapItem;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(claimed) = self.iter.next() {
            if claimed.0 > self.last {
                let cache = self.last;
                self.last = claimed.1;
                return Some((cache, claimed.0));
            }
            self.last = claimed.1;
        }
        None
    }
}

#[derive(Debug)]
pub enum MemoryMapperError {
    AlreadyOccupiedBy((usize, usize)), // contains the occupant region
    OutOfBound((usize, usize)),        // contains the valid Mapper region
}

// WARNING: these traits are not meant to be used only tinyvec::ArrayVec
#[derive(Clone, Default)]
pub struct MapArea {
    region: (usize, usize),
}

impl<'a> MapArea {
    // this function must be called only from inside MemoryMapper::claim() or MemoryMapper::force_free()
    fn new(region: (usize, usize)) -> Self {
        Self { region }
    }

    #[inline]
    fn validate<T>(&self, ptr: *const T) -> bool {
        ptr as usize >= self.region.0
            && ptr as usize + unsafe { mem::size_of_val_raw(ptr) } <= self.region.1
    }

    fn create_ptr<T>(&self, addr: usize) -> Option<*const T> {
        if self.validate(addr as *const T) {
            return Some(addr as *const T);
        }
        // invalid address
        None
    }

    unsafe fn get<T>(&self, ptr: *const T) -> Option<&'a T> {
        if self.validate(ptr) {
            return Some(&*ptr);
        }
        // invalid address
        None
    }
}

impl Drop for MapArea {
    // if MapArea is dropped, it can no longer be safely free'd. If the area is allocated
    // permanently its not much  of an issue, but otherwise it forces the use of the unsafe force_free
    fn drop(&mut self) {
        // default Self is not valid
        if self.region == (0, 0) {
            return;
        }
        log!(
            "[WARNING] Dropping MapArea handle for region {:016X} - {:016X}!\n",
            self.region.0,
            self.region.1
        )
    }
}

// ========== Implementations

// TABLE MEMORY MAPPER
// Simple implementation of MemoryMapper that uses a statically sized array(table) to store entries.
// WARNING: will panic if table gets full

const TABLE_SIZE: usize = 400; // max amount of entries
pub struct TableMemoryMapper {
    start: usize,
    end: usize,
    table: Mutex<[Option<(usize, usize)>; TABLE_SIZE]>,
}

pub struct TableMap {
    entries: [(usize, usize); TABLE_SIZE],
    count: usize,
    limit: usize,
}

impl Iterator for TableMap {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        match self.entries.get(self.count - 1) {
            Some(x) => {
                if self.count > self.limit {
                    None
                } else {
                    Some(*x)
                }
            }
            None => None,
        }
    }
}

impl MemoryMapper<TableMap> for TableMemoryMapper {
    unsafe fn manage(region: (usize, usize)) -> Self {
        Self {
            start: region.0,
            end: region.1,
            table: Mutex::new([None; TABLE_SIZE]),
        }
    }

    fn claim(&self, region: (usize, usize)) -> Result<MapArea, MemoryMapperError> {
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
                if (first < start && last > start) || (first > start && first < end) {
                    return Err(MemoryMapperError::AlreadyOccupiedBy((first, last)));
                }
            }
        }

        // get first empty slot to store our entry
        let mut i = 0;
        while table[i].is_some() {
            i += 1
        }
        // panic if no empty slot has been found
        if i >= TABLE_SIZE {
            panic!("Maximum number of MemoryMapper entries reached!");
        }

        table[i] = Some(region);
        Ok(MapArea::new((start, end - start)))
    }

    // WARNING: calling free() on a region that is still used may lead to undefind behaviour
    // returns true -> on success, false -> the target region has not been found
    fn free(&self, area: MapArea) {
        let mut table = self.table.lock();
        let mut ai = None;
        for (i, taken) in table.iter().enumerate() {
            if taken == &Some(area.region) {
                ai = Some(i);
                break;
            }
        }
        match ai {
            Some(i) => table[i] = None,
            None => panic!("Poisoned MapArea could not be freed!"),
        }
    }

    fn iter(&self) -> TableMap {
        let table = self.table.lock();
        let mut tm = TableMap {
            count: 0,
            entries: [(0, 0); TABLE_SIZE],
            limit: 0,
        };
        let mut i = 0;
        for slot in table.iter().flatten() {
            tm.entries[i] = *slot;
            i += 1;
        }
        tm.limit = i;
        tm
    }

    fn dimensions(&self) -> MapItem {
        (self.start, self.end)
    }
}
