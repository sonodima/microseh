/// Represents a system-specific exception code.
///
/// This enum encapsulates the various exception codes that can be returned by the
/// `GetExceptionCode` Windows API function.
///
/// See: <https://learn.microsoft.com/en-us/windows/win32/debug/getexceptioncode>

pub const INVALID_EXCEPTION_CODE: u32 = 0;

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExceptionCode {
    Invalid = INVALID_EXCEPTION_CODE,
    AccessViolation = 0xC0000005,
    ArrayBoundsExceeded = 0xC000008C,
    Breakpoint = 0x80000003,
    DataTypeMisalignment = 0x80000002,
    FltDenormalOperand = 0xC000008D,
    FltDivideByZero = 0xC000008E,
    FltInexactResult = 0xC000008F,
    FltInvalidOperation = 0xC0000090,
    FltOverflow = 0xC0000091,
    FltStackCheck = 0xC0000092,
    FltUnderflow = 0xC0000093,
    GuardPage = 0x80000001,
    IllegalInstruction = 0xC000001D,
    InPageError = 0xC0000006,
    IntDivideByZero = 0xC0000094,
    IntOverflow = 0xC0000095,
    InvalidDisposition = 0xC0000026,
    InvalidHandle = 0xC0000008,
    NonContinuableException = 0xC0000025,
    PrivilegedInstruction = 0xC0000096,
    SingleStep = 0x80000004,
    StackOverflow = 0xC00000FD,
    UnwindConsolidate = 0x80000029,
}

impl core::fmt::Display for ExceptionCode {
    /// Formats the exception code into a human-readable string.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to write to.
    ///
    /// # Returns
    ///
    /// Whether the formatting operation succeeded.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ExceptionCode::Invalid => write!(f, "invalid exception"),
            ExceptionCode::AccessViolation => write!(f, "the thread attempts to read from or write to a virtual address for which it does not have access"),
            ExceptionCode::ArrayBoundsExceeded => write!(f, "the thread attempts to access an array element that is out of bounds and the underlying hardware supports bounds checking"),
            ExceptionCode::Breakpoint => write!(f, "a breakpoint was encountered"),
            ExceptionCode::DataTypeMisalignment => write!(f, "the thread attempts to read or write data that is misaligned on hardware that does not provide alignment"),
            ExceptionCode::FltDenormalOperand => write!(f, "one of the operands in a floating point operation is denormal"),
            ExceptionCode::FltDivideByZero => write!(f, "the thread attempts to divide a floating point value by a floating point divisor of 0"),
            ExceptionCode::FltInexactResult => write!(f, "the result of a floating point operation cannot be represented exactly as a decimal fraction"),
            ExceptionCode::FltInvalidOperation => write!(f, "this exception represents any floating point exception not included in this list"),
            ExceptionCode::FltOverflow => write!(f, "the exponent of a floating point operation is greater than the magnitude allowed by the corresponding type"),
            ExceptionCode::FltStackCheck => write!(f, "the stack has overflowed or underflowed, because of a floating point operation"),
            ExceptionCode::FltUnderflow => write!(f, "the exponent of a floating point operation is less than the magnitude allowed by the corresponding type"),
            ExceptionCode::GuardPage => write!(f, "the thread accessed memory allocated with the PAGE_GUARD modifier"),
            ExceptionCode::IllegalInstruction => write!(f, "the thread tries to execute an invalid instruction"),
            ExceptionCode::InPageError => write!(f, "the thread tries to access a page that is not present, and the system is unable to load the page"),
            ExceptionCode::IntDivideByZero => write!(f, "the thread attempts to divide an integer value by an integer divisor of 0"),
            ExceptionCode::IntOverflow => write!(f, "the result of an integer operation creates a value that is too large to be held by the destination register"),
            ExceptionCode::InvalidDisposition => write!(f, "an exception handler returns an invalid disposition to the exception dispatcher"),
            ExceptionCode::InvalidHandle => write!(f, "the thread used a handle to a kernel object that was invalid"),
            ExceptionCode::NonContinuableException => write!(f, "the thread attempts to continue execution after a non-continuable exception occurs"),
            ExceptionCode::PrivilegedInstruction => write!(f, "the thread attempts to execute an instruction with an operation that is not allowed in the current computer mode"),
            ExceptionCode::SingleStep => write!(f, "a trace trap or other single instruction mechanism signals that one instruction is executed"),
            ExceptionCode::StackOverflow => write!(f, "the thread used up its stack"),
            ExceptionCode::UnwindConsolidate => write!(f, "a frame consolidation has been executed"),
        }
    }
}

impl From<u32> for ExceptionCode {
    /// Safely converts a raw `u32` exception code into an `ExceptionCode` variant.
    ///
    /// Unknown codes — values not in the enum — are mapped to `ExceptionCode::Invalid`
    /// to avoid the undefined behavior that would occur from transmuting an out-of-range
    /// discriminant into a `#[repr(u32)]` enum.
    fn from(code: u32) -> Self {
        match code {
            0xC0000005 => Self::AccessViolation,
            0xC000008C => Self::ArrayBoundsExceeded,
            0x80000003 => Self::Breakpoint,
            0x80000002 => Self::DataTypeMisalignment,
            0xC000008D => Self::FltDenormalOperand,
            0xC000008E => Self::FltDivideByZero,
            0xC000008F => Self::FltInexactResult,
            0xC0000090 => Self::FltInvalidOperation,
            0xC0000091 => Self::FltOverflow,
            0xC0000092 => Self::FltStackCheck,
            0xC0000093 => Self::FltUnderflow,
            0x80000001 => Self::GuardPage,
            0xC000001D => Self::IllegalInstruction,
            0xC0000006 => Self::InPageError,
            0xC0000094 => Self::IntDivideByZero,
            0xC0000095 => Self::IntOverflow,
            0xC0000026 => Self::InvalidDisposition,
            0xC0000008 => Self::InvalidHandle,
            0xC0000025 => Self::NonContinuableException,
            0xC0000096 => Self::PrivilegedInstruction,
            0x80000004 => Self::SingleStep,
            0xC00000FD => Self::StackOverflow,
            0x80000029 => Self::UnwindConsolidate,
            _ => Self::Invalid,
        }
    }
}
