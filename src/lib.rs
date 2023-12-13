#![cfg_attr(not(feature = "std"), no_std)]
use core::ffi::c_void;

mod code;
mod exception;
mod registers;

pub use code::ExceptionCode;
pub use exception::Exception;

type HandledProc = unsafe extern "system" fn(*mut c_void);

extern "C" {
    #[link_name = "HandlerStub"]
    fn handler_stub(proc: HandledProc, closure: *mut c_void, exception: *mut Exception) -> bool;
}

unsafe extern "system" fn handled_proc<F>(closure: *mut c_void)
where
    F: FnMut(),
{
    // Closure may be equal to std::ptr::null_mut() if the compiler optimized it away.
    // This also means that if you have some code that is optimized away, any exception it
    // contained will not get thrown.
    if let Some(closure) = closure.cast::<F>().as_mut() {
        closure();
    }
}

/// Executes a closure or function within a SEH-handled context.
///
/// # Arguments
///
/// * `closure` - The closure or function to be executed within the SEH-handled context.
///
/// # Returns
///
/// * `Ok(())` - If the closure executed without throwing any exceptions.
/// * `Err(Exception)` - If an exception occurred during the execution of the closure.
pub fn try_seh<F>(mut closure: F) -> Result<(), Exception>
where
    F: FnMut(),
{
    let mut exception = Exception::empty();
    let closure = &mut closure as *mut _ as *mut c_void;

    unsafe {
        match handler_stub(handled_proc::<F>, closure, &mut exception) {
            false => Err(exception),
            true => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::code::ExceptionCode;

    use super::*;

    #[test]
    #[cfg(feature = "std")]
    fn all_good() {
        let ex = try_seh(|| {
            let _ = *Box::new(1337);
        });

        assert_eq!(ex.is_ok(), true);
    }

    #[test]
    fn access_violation() {
        let ex = try_seh(|| unsafe {
            core::ptr::read_volatile::<i32>(0 as _);
        });

        assert_eq!(ex.is_err(), true);
        assert_eq!(ex.unwrap_err().code(), ExceptionCode::AccessViolation);
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn breakpoint() {
        let ex = try_seh(|| unsafe {
            core::arch::asm!("int3");
        });

        assert_eq!(ex.is_err(), true);
        assert_eq!(ex.unwrap_err().code(), ExceptionCode::Breakpoint);
    }

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn illegal_instruction() {
        let ex = try_seh(|| unsafe {
            core::arch::asm!("ud2");
        });

        assert_eq!(ex.is_err(), true);
        assert_eq!(ex.unwrap_err().code(), ExceptionCode::IllegalInstruction);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn illegal_instruction() {
        let ex = try_seh(|| unsafe {
            core::arch::asm!("udf #0");
        });

        assert_eq!(ex.is_err(), true);
        assert_eq!(
            ex.as_ref().unwrap_err().code(),
            ExceptionCode::IllegalInstruction
        );
    }

    #[test]
    #[cfg(target_arch = "x86")]
    fn reg_state_check() {
        let ex = try_seh(|| unsafe {
            core::arch::asm!("mov eax, 0xbadc0de", "ud2");
        });

        assert_eq!(ex.is_err(), true);
        assert_eq!(
            ex.as_ref().unwrap_err().code(),
            ExceptionCode::IllegalInstruction
        );

        assert_eq!(ex.unwrap_err().registers().eax(), 0xbadc0de);
    }

    #[test]
    #[cfg(target_arch = "x86_64")]
    fn reg_state_check() {
        let ex = try_seh(|| unsafe {
            core::arch::asm!("mov rax, 0xbadc0debabefffff", "ud2");
        });

        assert_eq!(ex.is_err(), true);
        assert_eq!(
            ex.as_ref().unwrap_err().code(),
            ExceptionCode::IllegalInstruction
        );

        assert_eq!(ex.unwrap_err().registers().rax(), 0xbadc0debabefffff);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn reg_state_check() {
        let ex = try_seh(|| unsafe {
            core::arch::asm!(
                "movz x0, #0xbadc, LSL 48",
                "movk x0, #0x0deb, LSL 32",
                "movk x0, #0xabef, LSL 16",
                "movk x0, #0xffff",
                "udf #0"
            );
        });

        assert_eq!(ex.is_err(), true);
        assert_eq!(
            ex.as_ref().unwrap_err().code(),
            ExceptionCode::IllegalInstruction
        );

        assert_eq!(ex.unwrap_err().registers().x0(), 0xbadc0debabefffff);
    }
}
