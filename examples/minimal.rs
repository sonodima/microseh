// Disable deny unconditional_panics

fn with_propagation() -> Result<(), microseh::Exception> {
    // ...

    microseh::try_seh(|| unsafe {
        std::ptr::read_volatile::<i32>(0 as _);
    })?;

    // Execution will not reach this point if an exception is thrown.

    // ...

    Ok(())
}

fn main() {
    if let Err(ex) = with_propagation() {
        println!("{:?}: {}", ex.address(), ex);
    }
}
