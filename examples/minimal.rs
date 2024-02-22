// Disable deny unconditional_panics

const INVALID_PTR: *mut i32 = core::mem::align_of::<i32>() as _;


fn with_propagation() -> Result<(), microseh::Exception> {
    // ...

    microseh::try_seh(|| unsafe {
        INVALID_PTR.read_volatile();
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
