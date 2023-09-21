use std::ffi::c_void;

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
            code: ExceptionCode::AccessViolation,
            address: std::ptr::null_mut(),
        }
    }

    pub fn code(&self) -> ExceptionCode {
        self.code
    }

    pub fn address(&self) -> *mut c_void {
        self.address
    }
}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code)
    }
}

impl std::error::Error for Exception {}
