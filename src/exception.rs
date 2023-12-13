use core::ffi::c_void;

use crate::{code::ExceptionCode, registers::Registers};

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Exception {
    code: ExceptionCode,
    address: *mut c_void,
    #[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
    registers: Registers,
}

impl Exception {
    pub(crate) fn empty() -> Self {
        Self {
            code: ExceptionCode::Invalid,
            address: core::ptr::null_mut(),
            registers: Registers::empty(),
        }
    }

    pub fn code(&self) -> ExceptionCode {
        self.code
    }

    pub fn address(&self) -> *mut c_void {
        self.address
    }

    #[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
    pub fn registers(&self) -> &Registers {
        &self.registers
    }
}

impl core::fmt::Display for Exception {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.code)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Exception {}
