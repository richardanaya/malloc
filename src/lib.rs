#![no_std]
use core::alloc::{GlobalAlloc, Layout};

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.mem_malloc(layout.size(),layout.align())
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        self.mem_free(ptr as usize)
    }
}

// Right now this allocator is just dumb and never frees

static START: usize = 0;
static mut END: usize = 0;
const USIZE_LENGTH:usize=  core::mem::size_of::<usize>();
const FLAG_ALLOCATED:usize = 1;
const FLAG_UNALLOCATED:usize = 0;

impl Allocator {
    #[inline]
    unsafe fn mem_malloc(&self, size: usize, _align: usize) -> *mut u8 {
        // find the start of a block
        let mut block_start = START;
        loop {
            //if we've reached the end of the line
            if block_start >= END {
                // start a block at the end
                block_start = END;
                break;
            }
            let flags = block_start as *const usize;
            let length = (block_start+USIZE_LENGTH) as *const usize;

            //if the current block we are on is unallocated and has room
            if *flags == FLAG_UNALLOCATED && *length <= size {
                // use it
                break;
            }
            block_start += *length+USIZE_LENGTH+USIZE_LENGTH;
        }
        if block_start == END {
            // create new block
            let mut p = block_start;
            let flags = p as *mut usize;
            *flags = FLAG_ALLOCATED;
            p += USIZE_LENGTH;
            let length = p as *mut usize;
            *length = size;
            p += USIZE_LENGTH;
            END += size+USIZE_LENGTH+USIZE_LENGTH;
            return p as *mut u8;
        } else {
            // reuse block
            // create new block
            let mut p = block_start;
            let flags = p as *mut usize;
            *flags = FLAG_ALLOCATED;
            p += USIZE_LENGTH + USIZE_LENGTH;
            return p as *mut u8;
        }
    }

    #[inline]
    unsafe fn mem_free(&self, loc: usize) {
        let block_start = (loc-USIZE_LENGTH-USIZE_LENGTH) as *mut usize;
        *block_start = FLAG_UNALLOCATED
    }
}
