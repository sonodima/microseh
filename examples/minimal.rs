// Disable deny unconditional_panics

use std::error::Error;

fn with_propagation() -> Result<(), Box<dyn Error>> {
    // ...

    microseh::try_seh(|| unsafe {
        *std::ptr::null_mut() = 0;
    })?;

    // Execution will not reach this point if an exception is thrown.

    // ...

    Ok(())
}

fn main() {
    if let Err(e) = with_propagation() {
        println!("exception: {}", e);
    }
}
