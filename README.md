# malloc

A memory allocator for web assembly.

* `#![no_std]`
* small footprint for smaller wasm
* simple to learn from
* works only on nightly

# Usage

```toml
[dependencies]
malloc = "0.0.4"
```

```rust
#![no_std]
#![feature(alloc_error_handler)]
extern crate alloc;

#[no_mangle]
pub fn main() -> () {
    // your code goes here
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
```
