<h1 align="center">MicroSEH 🔴</h1>

<div align="center">
  <a href="https://crates.io/crates/microseh"><img src="https://img.shields.io/crates/v/microseh.svg"/></a>
  <a href="https://github.com/sonodima/microseh/actions?workflow=CI">
    <img src="https://github.com/sonodima/microseh/workflows/CI/badge.svg"/>
  </a>
  <a href="https://crates.io/crates/microseh"><img src="https://img.shields.io/crates/d/microseh?color=pink"/></a>
  <img src="https://img.shields.io/badge/license-MIT-blue.svg"/>
</div>

<br>

> MicroSEH is a tiny library that implements Structured Exception Handling (SEH) in Rust and can catch
> and handle hardware exceptions.

## Why?

Hardware exceptions are a very powerful tool for specific use cases. One such use case is to
detect and handle illegal instructions at runtime.

## Implementation

It turns out that implementing SEH in pure Rust has its own issues (as seen in
[this article from NAMAZSO](https://engineering.zeroitlab.com/2022/03/13/rust-seh))

This library uses a different, simpler approach, which is to use a `C` stub that calls back into Rust, wrapping
the call in a `__try __except` block.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
microseh = "1.0"
```

**Minimal Example:** Dereference a null pointer without crashing the program, and return the handled exception.

```rust
fn guarded() -> Result<(), Box<dyn Error>> {
    microseh::try_seh(|| unsafe {
        std::ptr::read_volatile::<i32>(0 as _);
    })?;
}
```

**Accessing Exception Data:** You can obtain the address and register dump of an exception.

```rust
if let Err(ex) = microseh::try_seh(|| unsafe {
    // *debatable coding choices go here*
}) {
    println!("address: {:x}", ex.address());
    println!("rax: {:x}", ex.registers().rax());
}
```

_For additional examples and practical use cases, please visit the [examples](./examples) directory!_

## Portability

SEH is an extension to the C language developed by Microsoft, and it is exclusively available
on Windows when using Microsoft Visual C++ (MSVC).

MicroSEH is compatible with and has been tested on Windows platforms with the following
architectures: **x86**, **x86_64** and **aarch64**.
