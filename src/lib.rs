use libc::{c_int, c_void};

pub use exception::SEHException;

mod exception;

/// The type of a function that can be called by the `seh_stub` function.
type SEHCallback = unsafe extern "system" fn(*mut c_void);

extern "system" {
    /// Executes a function in an exception-handling block.<br><br>
    ///
    /// # Arguments
    /// * `callback` - Simple function that calls the guarded procedure in the SEH context.
    /// * `closure_ptr` - The procedure to execute in the exception-handling block.
    ///
    /// # Returns
    /// 0 if no exception was thrown, otherwise the exception code.
    fn seh_stub(
        callback: SEHCallback,
        closure_ptr: *mut c_void,
    ) -> c_int;
}

/// Internal function that calls the guarded procedure in the SEH context.<br>
/// This function is called by the `seh_stub` FFI function.<br><br>
///
/// # Arguments
/// * `closure_ptr` - The procedure to execute in the exception-handling block.
unsafe extern "system" fn seh_callback<F>(closure_ptr: *mut c_void)
    where
        F: FnMut(),
{
    // Convert the raw pointer passed by the C stub function.
    let closure = &mut *(closure_ptr as *mut F);

    // Call the closure passed to try_seh.
    closure();
}

/// Executes a function in a structure-exception-handled block.<br>
/// This is useful for executing code that may throw an exception, and crash
/// the program. (such as a SEGFAULT)<br><br>
///
/// # Arguments
/// * `closure` - The procedure to execute in the exception-handling block.
///
/// # Returns
/// Some if no exception was thrown, None if an exception was thrown.
pub fn try_seh<F>(mut closure: F) -> Result<(), SEHException>
    where
        F: FnMut(),
{
    // Get the raw pointer to the closure.
    let closure_ptr = &mut closure as *mut _ as *mut c_void;

    // Call the C stub function.
    // This function will call the `seh_callback` function in an SEH block,
    // passing the raw pointer to the closure.
    // The `seh_callback` function will then call the closure.
    match unsafe { seh_stub(seh_callback::<F>, closure_ptr) } {
        0 => Ok(()),
        code => Err(SEHException::from(code)),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that the `try_seh` function does not return an error when no
    /// exception is thrown.
    #[test]
    fn no_exception() {
        assert!(
            try_seh(|| {
                let _ = *Box::new(420);
            }).is_ok()
        );
    }

    /// Tests that the `try_seh` function can handle access violations.
    #[test]
    fn access_violation() {
        assert!(
            try_seh(|| unsafe {
                let _data = *Box::from_raw(std::ptr::null_mut::<u8>());
            }).is_err()
        );

        assert!(
            try_seh(|| unsafe {
                let _data = std::ptr::read_volatile(std::ptr::null::<u8>());
            }).is_err()
        );
    }
}
