use core::ffi::c_void;

use crate::code::ExceptionCode;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Exception {
    code: ExceptionCode,
    address: *mut c_void,
}

impl Exception {
    pub(crate) fn empty() -> Self {
        Self {
            code: ExceptionCode::Invalid,
            address: core::ptr::null_mut(),
        }
    }

    pub fn code(&self) -> ExceptionCode {
        self.code
    }

    pub fn address(&self) -> *mut c_void {
        self.address
    }
}

impl core::fmt::Display for Exception {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.code)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Exception {}
