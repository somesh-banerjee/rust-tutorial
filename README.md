# Learning Rust

This repository contains rust programs for too many linked list that I have written while learning Rust. The programs are based on the book [Too Many Linked Lists](https://rust-unofficial.github.io/too-many-lists/).

### How to run the programs

```bash
cargo build
cargo test
```

### Notes for self

- Revisit lifetime in Rust
- Arc for thread safety is not clear, probably will not be used in common use cases
- Stack borrow: only understood that whatever is in top of the stack is live, and when we access some element from below, everything above it is popped out. Need more clarity on raw pointers using examples. But overall, this is similar to C/C++ memory management.
