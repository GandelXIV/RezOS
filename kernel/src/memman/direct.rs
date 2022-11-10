// This module handles direct memory region claimation
use core::marker::PhantomData;
use core::slice;

// TODO: free when this drops
pub struct Ptr<'a, T: ?Sized> {
    raw: *mut T,
    _phantom: PhantomData<&'a T>,
}

trait MemoryMapper {
    // SAFETY! region must adhere to the following:
    // - must have read & write priviliges for ring0
    // - must not hold any other already used structures
    // - must not be managed by another MemoryMapper
    unsafe fn manage(region: (usize, usize)) -> Self;
    // in case the region is occupied, returns Err(occupant region)
    fn claim(&mut self, region: (usize, usize)) -> Result<Ptr<[u8]>, (usize, usize)>;
    // returns true on success, and false if the region could not be found
    fn free(&mut self, region: (usize, usize)) -> bool;
}


// TABLE MEMORY MAPPER
// Simple implementation of MemoryMapper that uses a statically sized table to store entries. 
const TABLE_SIZE: usize = 1024; // max amount of entries in the memory map
struct TableMemoryMapper {
    start: usize,
    end: usize,
    table: [Option<(usize, usize)>; TABLE_SIZE],
}

impl MemoryMapper for TableMemoryMapper {
    unsafe fn manage(region: (usize, usize)) -> Self {
       Self {
           start: region.0,
           end: region.1,
           table: [None; TABLE_SIZE],
       } 
    }

    fn claim(&mut self, region: (usize, usize)) -> Result<Ptr<[u8]>, (usize, usize)> {
        let (start, end) = region;
        for taken in self.table.iter() {
            if let Some((first, last)) = *taken {
                if (first <= start && last >= start) || (first >= start && first <= end) {
                    return Err((first, last));
                }
            }
        }

        let mut i = 0;
        while let Some(_) = self.table[i] {
            i += 1
        }
        self.table[i] = Some(region);
        Ok(Ptr {
            raw: unsafe { slice::from_raw_parts_mut(start as *mut u8, end - start) as *mut [u8] },
            _phantom: PhantomData,
        })
    }

    fn free(&mut self, region: (usize, usize)) -> bool {
        for (i, taken) in self.table.iter().enumerate() {
            if taken == &Some(region) {
                self.table[i] = None;
                return true;
            }
        }
        false
    }
}
