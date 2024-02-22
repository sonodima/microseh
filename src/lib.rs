#![cfg_attr(not(feature = "std"), no_std)]
use core::ffi::c_void;

mod code;
mod exception;
mod registers;

pub use code::ExceptionCode;
pub use exception::Exception;

type HandledProc = unsafe extern "system" fn(*mut c_void);

const MS_CATCHED: u32 = 0x1;
const MS_DISABLED: u32 = 0x2;

extern "C" {
    #[link_name = "HandlerStub"]
    fn handler_stub(proc: HandledProc, closure: *mut c_void, exception: *mut Exception) -> u32;
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

/// Executes the provided closure in a context where exceptions are handled, catching any\
/// hardware exceptions that occur.
///
/// # Arguments
///
/// * `closure` - The closure or function to be executed within the handled context.
///
/// # Returns
///
/// * `Ok(())` - If the closure executed without throwing any exceptions.
/// * `Err(Exception)` - If an exception occurred during the execution of the closure.
/// 
/// # Examples
///
/// ```
/// use microseh::try_seh;
///
/// if let Err(e) = try_seh(|| unsafe {
///     core::ptr::read_volatile(core::mem::align_of::<i32>() as *const i32);
/// }) {
///     println!("an exception occurred: {:?}", e);
/// }
/// ```
/// 
/// # Caveats
/// 
/// If an exception occours within the closure, resources that require cleanup via\
/// the `Drop` trait, may not be properly released.
/// 
/// As a rule of thumb, it's recommended not to define resources that implement\
/// the `Drop` trait inside the closure. Instead, allocate and manage these resources\
/// outside the closure, ensuring proper cleanup even if an exception occurs.
/// 
/// # Panics
/// 
/// If exception handling is disabled in the build, which occurs when the library is\
/// not built on Windows with Microsoft Visual C++.
pub fn try_seh<F>(mut closure: F) -> Result<(), Exception>
where
    F: FnMut(),
{
    let mut exception = Exception::empty();
    let closure = &mut closure as *mut _ as *mut c_void;

    unsafe {
        match handler_stub(handled_proc::<F>, closure, &mut exception) {
            MS_CATCHED => Err(exception),
            MS_DISABLED => panic!("exception handling is not supported in this build of microseh"),
            /* MS_SUCCEEDED */ _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INVALID_PTR: *mut i32 = core::mem::align_of::<i32>() as _;


    #[test]
    #[cfg(feature = "std")]
    fn all_good() {
        let ex = try_seh(|| {
            let _ = *Box::new(1337);
        });

        assert_eq!(ex.is_ok(), true);
    }

    #[test]
    fn access_violation_rs() {
        let ex = try_seh(|| unsafe {
            INVALID_PTR.read_volatile();
        });

        assert_eq!(ex.is_err(), true);
        assert_eq!(ex.unwrap_err().code(), ExceptionCode::AccessViolation);
    }


    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn access_violation_asm() {
        let ex = try_seh(|| unsafe {
            core::arch::asm!("mov eax, DWORD PTR [0]");
        });

        assert_eq!(ex.is_err(), true);
        assert_eq!(ex.unwrap_err().code(), ExceptionCode::AccessViolation);
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn access_violation_asm() {
        let ex = try_seh(|| unsafe {
            core::arch::asm!("ldr x0, xzr");
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
