# malloc

A memory allocator for web assembly.

* `#![no_std]` and stable
* small footprint for smaller wasm
* simple to learn from


# Usage

```toml
[dependencies]
malloc = "0.0.1"
```

```rust
#[global_allocator]
static ALLOCATOR:malloc::Allocator = malloc::Allocator;
```
