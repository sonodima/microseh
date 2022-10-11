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
> Having a way to handle hardware exceptions is useful, especially when you're interfacing with other C binaries.<br>
> There may be some cases where a hardware exception is not a fatal error, and can be resolved by the program.
>
> The most basic application of such concept is to catch a segmentation fault when accessing an invalid pointer.

## Implementation

It turns out that implementing SEH in safe Rust may not look that straightforward at first glance (as seen
in [this post](https://engineering.zeroitlab.com/2022/03/13/rust-seh))

This library uses a different, simpler approach, which is to use a `C` stub that calls back into Rust,
wrapping the call in a `__try __except` block.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
microseh = { git = "https://github.com/sonodima/microseh" }
```

> <b>Example:</b> De-referencing a null pointer without crashing the program.

```rust
fn guarded() -> Result<(), Box<dyn Error>> {
    microseh::try_catch(|| {
        *std::ptr::null::<i32>();
    })?;
}
```

## Portability

SEH is a Microsoft extension to the C language, so it's only available on Windows with MSVC.
