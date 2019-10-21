#![no_std]
use core::alloc::{GlobalAlloc, Layout};

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.mem_malloc(layout.size())
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        self.mem_free(ptr as usize)
    }
}

// Right now this allocator is just dumb and never frees

static mut START:usize = 0;

impl Allocator {
    #[inline]
    unsafe fn mem_malloc(&self, size: usize) -> *mut u8 {
        let p = START;
        START += size;
        return p as *mut u8;
    }

    #[inline]
    unsafe fn mem_free(&self, _loc: usize) {
        // DO NOTHING
    }
}