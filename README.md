# RVV in Rust STD Demonstration

This illustrates a very early proof-of-concept for what a type-state based 
solution for implementing the RISC-V Vector Extension might look like.

Currently, there is one example: `vadd` implemented in
[`examples/vadd.rs`](./examples/vadd.rs). This runs fine with `qemu-riscv64`.

```bash
# Run example 
cargo +nightly run --example vadd

# Inspect result assembly
llvm-objdump -d target/riscv64gc-unknown-linux-gnu/debug/examples/vadd
```

This was made for the [scalable representation RFC](https://github.com/rust-lang/rfcs/pull/3268).
