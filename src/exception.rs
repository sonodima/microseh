use core::ffi::c_void;

use crate::code::{ExceptionCode, Register};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Exception {
    code: ExceptionCode,
    address: *mut c_void,

    #[cfg(target_arch = "x86_64")]
    regs: [usize; Register::Count as usize],

    #[cfg(target_arch = "x86")]
    regs: [usize; Register::Count as usize],
}

impl Exception {
    pub(crate) fn empty() -> Self {
        Self {
            code: ExceptionCode::Invalid,
            address: core::ptr::null_mut(),

            #[cfg(target_arch = "x86_64")]
            regs: [0; Register::Count as usize],

            #[cfg(target_arch = "x86")]
            regs: [0; Register::Count as usize],
        }
    }

    pub fn code(&self) -> ExceptionCode {
        self.code
    }

    pub fn address(&self) -> *mut c_void {
        self.address
    }

    pub fn register(&self, reg: Register) -> usize {
        self.regs[reg as usize]
    }

    pub fn registers(&self) -> &[usize] {
        &self.regs
    }
}

impl core::fmt::Display for Exception {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.code)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Exception {}
