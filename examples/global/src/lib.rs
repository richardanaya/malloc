#![no_std]
#![feature(alloc_error_handler)]
extern crate alloc;
use js_ffi::*;

struct Test {
    fn_log: f64,
}

impl Default for Test {
    fn default() -> Self {
        Test {
            fn_log: register("console.log"),
        }
    }
}

impl Test {
    fn log(&self, s: &str) {
        call_1(UNDEFINED, self.fn_log, TYPE_STRING, to_js_string(s));
    }
}

#[no_mangle]
pub fn main() -> () {
    let t = globals::get::<Test>().lock();
    t.log("hello world")
}

#[global_allocator]
static ALLOCATOR:malloc::Allocator = malloc::Allocator;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[alloc_error_handler]
fn oom(_: core::alloc::Layout) -> ! {
    loop {}
}
