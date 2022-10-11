<h1 align="center">MicroSEH ❌</h1>

<div align="center">
  <a href="https://github.com/sonodima/microseh/actions?workflow=CI">
    <img src="https://github.com/sonodima/microseh/workflows/CI/badge.svg"/>
  </a>

  <img src="https://img.shields.io/badge/license-MIT-blue.svg"/>
</div>

<br>

> MicroSEH is a small library for Structured Exception Handling (SEH) in Rust that can catch
> and handle hardware exceptions.

> Isn't Rust supposed to be safe?<br>
> Well, yes, but as soon as you enter the unsafe world, you can do anything you want.
>
> Having a way to handle hardware exceptions is
> still useful, especially when you're interfacing with other C binaries.<br>
> Rust makes the assumption that once an exception is thrown, the program is in an undefined state.
> 
> For example, you can catch a `SIGSEGV` and continue execution.

## Implementation

It turns out that implementing SEH in safe Rust may not look that straightforward at first glance (as seen in [this post](https://engineering.zeroitlab.com/2022/03/13/rust-seh))

This library uses a different, simpler approach, which is to use a `C` stub that calls back into Rust,
wrapping the call in a `__try __except` block.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
microseh = { git = "github.com/sonodima/microseh" }
```

> <b>Example:</b> De-referencing a null pointer without crashing the program.

```rust
fn main() {
    microseh::try_catch(|| {
        *std::ptr::null::<i32>();
    })?;
}
```

## Portability

SEH is a Microsoft extension to the C language, so it's only available on Windows with MSVC.
