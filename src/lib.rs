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

static START: usize = 0;
static mut END: usize = 0;
const HEADER_SIZE:usize=  core::mem::size_of::<Header>();
const FLAG_ALLOCATED:usize = 1;
const FLAG_UNALLOCATED:usize = 0;

struct Header {
    flags:usize,
    size:usize
}

impl Allocator {
    #[inline]
    unsafe fn mem_malloc(&self, size: usize, align: usize) -> *mut u8 {
        // find the start of a block
        let mut block_start = START;
        while block_start < END {
            let header = &mut *(block_start as *mut Header);
            if header.flags == FLAG_UNALLOCATED && header.size <= size {
                header.flags = FLAG_ALLOCATED;
                return (block_start+HEADER_SIZE) as *mut u8;
            }
            block_start += HEADER_SIZE+header.size;
        }

        // create a new block at the end if we didn't find
        // an allocated block that is available and right size
        let mut header = &mut *(END as *mut Header);
        header.flags = FLAG_ALLOCATED;
        header.size = size+8;
        END += HEADER_SIZE+header.size;
        (block_start+HEADER_SIZE) as *mut u8
    }

    #[inline]
    unsafe fn mem_free(&self, location: usize) {
        // find allocation block and mark as unallocated
        let mut header = &mut *((location-HEADER_SIZE) as *mut Header);
        header.flags = FLAG_UNALLOCATED;
    }
}
