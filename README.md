<h1 align="center">MicroSEH 🔴</h1>

<div align="center">
  <a href="https://crates.io/crates/microseh"><img src="https://img.shields.io/crates/v/microseh.svg?color=edae00&style=for-the-badge"/></a>
  <a href="https://github.com/sonodima/microseh/actions/workflows/ci.yml"> 
    <img src="https://img.shields.io/github/actions/workflow/status/sonodima/microseh/ci.yml?style=for-the-badge&label=CI%20Status"/>
  </a>
  <a href="https://crates.io/crates/microseh"><img src="https://img.shields.io/crates/d/microseh?color=pink&style=for-the-badge"/></a>
  <img src="https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge"/>
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
microseh = "1.2"
```

**Minimal Example:** Dereference an invalid pointer without crashing the program, and return the handled exception.

```rust
fn guarded() -> Result<(), microseh::Exception> {
    microseh::try_seh(|| unsafe {
        // Read from an unallocated memory region. (we create an aligned not-null
        // pointer to skip the checks in read_volatile that would raise a panic)
        core::ptr::read_volatile(core::mem::align_of::<i32>() as *const i32);
    })
}
```

**Accessing Exception Data:** You can obtain the address and register dump of an exception.

```rust
if let Err(ex) = microseh::try_seh(|| unsafe {
    // *questionable life choices go here*
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

When building for other unsupported platforms, the library will disable exception
handling and panic when `try_seh` is called.

## Usage on Kernel Drivers

This library can compile to `no_std` and supports running in Windows Kernel Drivers using
Microsoft's [windows-drivers-rs](https://github.com/microsoft/windows-drivers-rs) project.

## Cross-Compiling

Cross-compiling for Windows is possible with full support for SEH using the
[cargo-xwin](https://github.com/rust-cross/cargo-xwin) project.

## License

This work is released under the MIT license. A copy of the license is provided in the
[LICENSE](./LICENSE) file.
