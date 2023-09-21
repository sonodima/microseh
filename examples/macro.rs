// Disable deny unconditional_panics

use std::error::Error;

use microseh::try_seh;

unsafe fn with_macro() -> Result<(), Box<dyn Error>> {
    // ...

    // This is the same as doing microseh::try_seh(|| { ... })?;
    try_seh!({
        *std::ptr::null_mut() = 0;
    });

    // Execution will not reach this point if an exception is thrown.

    // ...

    Ok(())
}

fn main() {
    unsafe {
        if let Err(e) = with_macro() {
            println!("exception: {}", e);
        }
    }
}
