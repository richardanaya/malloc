#![no_std]
#![feature(alloc_error_handler)]
extern crate alloc;
use crate::alloc::string::ToString;
use js_ffi::*;
use alloc::format;

#[no_mangle]
pub fn main() -> () {
    let log = register("console.log");
    let s1 = "hello world".to_string();
    let s2 = "foo".to_string();
    call_1(UNDEFINED, log, TYPE_STRING, to_js_string(&s1));
    call_1(UNDEFINED, log, TYPE_STRING, to_js_string(&s2));
    call_1(UNDEFINED, log, TYPE_STRING, to_js_string(&format!("{}",42).to_string()));
    call_1(UNDEFINED, log, TYPE_STRING, to_js_string(&format!("{}",&s1 as *const _ as usize).to_string()));
    call_1(UNDEFINED, log, TYPE_STRING, to_js_string(&format!("{:?}",&s2 as *const _  as usize).to_string()));

    {
        let s3 = "hello world".to_string();
        call_1(UNDEFINED, log, TYPE_STRING, to_js_string(&format!("{}",&s3 as *const _ as usize).to_string()));
    }
    {
        let s3 = "hello world".to_string();
        call_1(UNDEFINED, log, TYPE_STRING, to_js_string(&format!("{}",&s3 as *const _ as usize).to_string()));
    }

    let s4 = "hello world".to_string();
    let s5 = "hello world".to_string();
    call_1(UNDEFINED, log, TYPE_STRING, to_js_string(&format!("{}",&s4 as *const _ as usize).to_string()));
    call_1(UNDEFINED, log, TYPE_STRING, to_js_string(&format!("{}",&s5 as *const _ as usize).to_string()));
    core::mem::drop(s4);
    let s6 = "hello world".to_string();
    call_1(UNDEFINED, log, TYPE_STRING, to_js_string(&format!("{}",&s6 as *const _ as usize).to_string()));
    call_1(UNDEFINED, log, TYPE_STRING, to_js_string(&format!("{}",&s5 as *const _ as usize).to_string()));
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