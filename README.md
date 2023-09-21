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

## Implementation

It turns out that implementing SEH in pure Rust has its own issues (as seen in
[this article from NAMAZSO](https://engineering.zeroitlab.com/2022/03/13/rust-seh))

This library uses a different, simpler approach, which is to use a `C` stub that calls back into Rust, wrapping
the call in a `__try __except` block.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
microseh = "0.2"
```

__Minimal Example:__ Dereference a null pointer without crashing the program, and return the handled exception.

```rust
fn guarded() -> Result<(), Box<dyn Error>> {
    microseh::try_seh(|| unsafe {
        *std::ptr::null::<i32>();
    })?;
}
```

_NOTE: Compiler optimizations may interfere with the closure. This example requires `opt-level=0` , otherwise
no exception would actually get thrown._

## Portability

SEH is a Microsoft extension to the C language, so it's only available on Windows with MSVC.
