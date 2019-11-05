#![no_std]
#![feature(alloc_error_handler)]
extern crate alloc;
use alloc::boxed::Box;

#[no_mangle]
pub fn main() -> usize {
    let b = Box::into_raw(Box::new(1)) as usize;
    let v = Box::into_raw(Box::new(42)) as usize;
    if v > b {
        return v;
    }
    666
}

#[global_allocator]
static ALLOCATOR: malloc::Allocator = malloc::Allocator;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    loop {}
}
