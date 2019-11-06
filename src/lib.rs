#![no_std]
use core::alloc::{GlobalAlloc, Layout};
use core::arch::wasm32;

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.mem_malloc(layout.size(),layout.align())
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.mem_free(ptr as usize,layout.size())
    }
}

// start and end of known memory
static START: usize = 0;
static mut END: usize = 0;

// number of pages we know exist
static mut NUM_PAGES: usize = 0;

// sizes of things
const HEADER_SIZE:usize=  core::mem::size_of::<Header>();
const USIZE_SIZE:usize=  core::mem::size_of::<usize>();
const PAGE_SIZE:usize = 65_536;

// flags to represent state of memory blocks
const FLAG_ALLOCATED:usize = 1;
const FLAG_UNALLOCATED:usize = 2;

// struct to repesent memory block header
struct Header {
    flags:usize,
    size:usize
}

/*

 MEMORY BLOCK

 [flags, size, padding_to_get_aligned_data, aligned_start_of_data, padding_size ]

*/

#[inline]
fn round_up_to_nearest_multiple(i:usize,multiple:usize) -> usize {
    ((i + multiple - 1) / multiple) * multiple
}

impl Allocator {
    #[inline]
    unsafe fn mem_malloc(&self, size: usize, align: usize) -> *mut u8 {
        // lets look through existing memory blocks
        let mut block_start = START;
        while block_start < END {
            let header = &mut *(block_start as *mut Header);
            // is this block unallocated
            if header.flags == FLAG_UNALLOCATED {
                // get the padding it would take to make data_start aligned
                let data_start = block_start+HEADER_SIZE;
                let aligned_data_start = round_up_to_nearest_multiple(data_start,align);
                let padding = aligned_data_start-data_start;
                let full_data_size = padding+header.size+USIZE_SIZE;
                // is there room in unallocated block?
                if full_data_size <= size {
                    // mark as allocated
                    header.flags = FLAG_ALLOCATED;
                    // store padding right after data so we can find it when we free
                    let padding_ptr = (aligned_data_start+header.size) as *mut usize;
                    *padding_ptr = padding;
                    // lets grow memory if we need to for this memory block
                    // by looking at the end and seeing if the page count
                    // is greater than current number of known pages
                    let cur_page = (aligned_data_start+header.size+USIZE_SIZE)/PAGE_SIZE;
                    while cur_page > NUM_PAGES {
                        wasm32::memory_grow(0, 1);
                        NUM_PAGES += 1;
                    }
                    // return aligned start of data
                    return aligned_data_start as *mut u8;
                }
            }
            // go to next block
            block_start += HEADER_SIZE+header.size+USIZE_SIZE;
        }

        // create a new block at the end
        let mut header = &mut *(END as *mut Header);
        let data_start = block_start+HEADER_SIZE;
        let aligned_data_start = round_up_to_nearest_multiple(data_start,align);
        let padding = aligned_data_start-data_start;
        let padding_ptr = (aligned_data_start+header.size) as *mut usize;
        *padding_ptr = padding;
        let full_data_size = padding+size;
        header.flags = FLAG_ALLOCATED;
        header.size = full_data_size;
        // increase size of known memory
        END += HEADER_SIZE+header.size+USIZE_SIZE;
        // lets grow memory if we need to for this memory block
        // by looking at the end and seeing if the page count
        // is greater than current number of known pages
        let cur_page = (aligned_data_start+header.size+USIZE_SIZE)/PAGE_SIZE;
        while cur_page > NUM_PAGES {
            wasm32::memory_grow(0, 1);
            NUM_PAGES += 1;
        }
        // return aligned start of data
        aligned_data_start as *mut u8
    }

    #[inline]
    unsafe fn mem_free(&self, location: usize, size:usize) {
        // since we are memory aligned, header may not be exactly behind data
        // get the padding count we stored right after data during allocation
        let padding_ptr = (location+size) as *mut usize;
        let padding = *padding_ptr;
        // use it to figure out where the header is relative to aliged data start
        let mut header = &mut *((location-HEADER_SIZE-padding) as *mut Header);
        // mark as unallocated
        header.flags = FLAG_UNALLOCATED;
    }
}
