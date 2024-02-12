// Disable deny unconditional_panics

fn with_propagation() -> Result<(), microseh::Exception> {
    // ...

    microseh::try_seh(|| unsafe {
        core::ptr::null::<i32>().read_volatile();
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
