use malloc::*;

#[global_allocator]
static GLOBAL: Allocator = Allocator;

fn main() {
    println!("Hello, world!");
}
