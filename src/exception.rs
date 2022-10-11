use std::fmt::Display;
use libc::c_int;
use windows_sys::Win32::Foundation::*;

/// See: https://learn.microsoft.com/en-us/windows/win32/debug/getexceptioncode
#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum SEHException {
    AccessViolation = EXCEPTION_ACCESS_VIOLATION,
    ArrayBoundsExceeded = EXCEPTION_ARRAY_BOUNDS_EXCEEDED,
    Breakpoint = EXCEPTION_BREAKPOINT,
    DataTypeMisalignment = EXCEPTION_DATATYPE_MISALIGNMENT,
    FLTDenormalOperand = EXCEPTION_FLT_DENORMAL_OPERAND,
    FLTDivideByZero = EXCEPTION_FLT_DIVIDE_BY_ZERO,
    FLTInexactResult = EXCEPTION_FLT_INEXACT_RESULT,
    FLTInvalidOperation = EXCEPTION_FLT_INVALID_OPERATION,
    FLTOverflow = EXCEPTION_FLT_OVERFLOW,
    FLTStackCheck = EXCEPTION_FLT_STACK_CHECK,
    FLTUnderflow = EXCEPTION_FLT_UNDERFLOW,
    GuardPage = EXCEPTION_GUARD_PAGE,
    IllegalInstruction = EXCEPTION_ILLEGAL_INSTRUCTION,
    InPageError = EXCEPTION_IN_PAGE_ERROR,
    IntDivideByZero = EXCEPTION_INT_DIVIDE_BY_ZERO,
    IntOverflow = EXCEPTION_INT_OVERFLOW,
    InvalidDisposition = EXCEPTION_INVALID_DISPOSITION,
    InvalidHandle = EXCEPTION_INVALID_HANDLE,
    NonContinuableException = EXCEPTION_NONCONTINUABLE_EXCEPTION,
    PrivilegedInstruction = EXCEPTION_PRIV_INSTRUCTION,
    SingleStep = EXCEPTION_SINGLE_STEP,
    StackOverflow = EXCEPTION_STACK_OVERFLOW,
    UnwindConsolidate = STATUS_UNWIND_CONSOLIDATE,
}

impl From<c_int> for SEHException {
    fn from(err: c_int) -> Self {
        unsafe { std::mem::transmute(err) }
    }
}

impl std::error::Error for SEHException {}

impl Display for SEHException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SEHException::AccessViolation => write!(f, "the thread attempts to read from or write to a virtual address for which it does not have access"),
            SEHException::ArrayBoundsExceeded => write!(f, "the thread attempts to access an array element that is out of bounds and the underlying hardware supports bounds checking"),
            SEHException::Breakpoint => write!(f, "a breakpoint was encountered"),
            SEHException::DataTypeMisalignment => write!(f, "the thread attempts to read or write data that is misaligned on hardware that does not provide alignment"),
            SEHException::FLTDenormalOperand => write!(f, "one of the operands in a floating point operation is denormal. A denormal value is one that is too small to represent as a standard floating point value"),
            SEHException::FLTDivideByZero => write!(f, "the thread attempts to divide a floating point value by a floating point divisor of 0 (zero)"),
            SEHException::FLTInexactResult => write!(f, "the result of a floating point operation cannot be represented exactly as a decimal fraction"),
            SEHException::FLTInvalidOperation => write!(f, "this exception represents any floating point exception not included in this list"),
            SEHException::FLTOverflow => write!(f, "the exponent of a floating point operation is greater than the magnitude allowed by the corresponding type"),
            SEHException::FLTStackCheck => write!(f, "the stack has overflowed or underflowed, because of a floating point operation"),
            SEHException::FLTUnderflow => write!(f, "the exponent of a floating point operation is less than the magnitude allowed by the corresponding type"),
            SEHException::GuardPage => write!(f, "the thread accessed memory allocated with the PAGE_GUARD modifier"),
            SEHException::IllegalInstruction => write!(f, "the thread tries to execute an invalid instruction"),
            SEHException::InPageError => write!(f, "the thread tries to access a page that is not present, and the system is unable to load the page"),
            SEHException::IntDivideByZero => write!(f, "the thread attempts to divide an integer value by an integer divisor of 0 (zero)"),
            SEHException::IntOverflow => write!(f, "the result of an integer operation creates a value that is too large to be held by the destination register"),
            SEHException::InvalidDisposition => write!(f, "an exception handler returns an invalid disposition to the exception dispatcher"),
            SEHException::InvalidHandle => write!(f, "the thread used a handle to a kernel object that was invalid"),
            SEHException::NonContinuableException => write!(f, "the thread attempts to continue execution after a non-continuable exception occurs"),
            SEHException::PrivilegedInstruction => write!(f, "the thread attempts to execute an instruction with an operation that is not allowed in the current computer mode"),
            SEHException::SingleStep => write!(f, "a trace trap or other single instruction mechanism signals that one instruction is executed"),
            SEHException::StackOverflow => write!(f, "the thread uses up its stack"),
            SEHException::UnwindConsolidate => write!(f, "a frame consolidation has been executed"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ensure that the size of the SEHException enum is the same as the size of a c_int.
    /// This is important because the exception code is a c_int.
    #[test]
    fn exception_size() {
        assert_eq!(std::mem::size_of::<SEHException>(), std::mem::size_of::<c_int>());
    }
}
