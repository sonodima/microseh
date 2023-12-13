#[cfg(target_arch = "x86")]
const NUM_REGISTERS: usize = 9;
#[cfg(target_arch = "x86_64")]
const NUM_REGISTERS: usize = 17;
#[cfg(target_arch = "aarch64")]
const NUM_REGISTERS: usize = 33;

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Registers {
    list: [usize; NUM_REGISTERS],
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64"))]
impl Registers {
    pub fn empty() -> Self {
        Self {
            list: [0; NUM_REGISTERS],
        }
    }

    pub fn list(&self) -> &[usize] {
        &self.list
    }
}

macro_rules! get_reg {
    ($reg:ident, $index:expr) => {
        #[inline]
        pub fn $reg(&self) -> usize {
            self.list[$index]
        }
    };
}

#[cfg(target_arch = "x86")]
impl Registers {
    get_reg!(eax, 0);
    get_reg!(ecx, 1);
    get_reg!(edx, 2);
    get_reg!(ebx, 3);
    get_reg!(esp, 4);
    get_reg!(ebp, 5);
    get_reg!(esi, 6);
    get_reg!(edi, 7);
    get_reg!(eip, 8);
}

#[cfg(target_arch = "x86_64")]
impl Registers {
    get_reg!(rax, 0);
    get_reg!(rcx, 1);
    get_reg!(rdx, 2);
    get_reg!(rbx, 3);
    get_reg!(rsp, 4);
    get_reg!(rbp, 5);
    get_reg!(rsi, 6);
    get_reg!(rdi, 7);
    get_reg!(r8, 8);
    get_reg!(r9, 9);
    get_reg!(r10, 10);
    get_reg!(r11, 11);
    get_reg!(r12, 12);
    get_reg!(r13, 13);
    get_reg!(r14, 14);
    get_reg!(r15, 15);
    get_reg!(rip, 16);
}

#[cfg(target_arch = "aarch64")]
impl Registers {
    get_reg!(x0, 0);
    get_reg!(x1, 1);
    get_reg!(x2, 2);
    get_reg!(x3, 3);
    get_reg!(x4, 4);
    get_reg!(x5, 5);
    get_reg!(x6, 6);
    get_reg!(x7, 7);
    get_reg!(x8, 8);
    get_reg!(x9, 9);
    get_reg!(x10, 10);
    get_reg!(x11, 11);
    get_reg!(x12, 12);
    get_reg!(x13, 13);
    get_reg!(x14, 14);
    get_reg!(x15, 15);
    get_reg!(x16, 16);
    get_reg!(x17, 17);
    get_reg!(x18, 18);
    get_reg!(x19, 19);
    get_reg!(x20, 20);
    get_reg!(x21, 21);
    get_reg!(x22, 22);
    get_reg!(x23, 23);
    get_reg!(x24, 24);
    get_reg!(x25, 25);
    get_reg!(x26, 26);
    get_reg!(x27, 27);
    get_reg!(x28, 28);
    get_reg!(fp, 29);
    get_reg!(lr, 30);
    get_reg!(sp, 31);
    get_reg!(pc, 32);
}
