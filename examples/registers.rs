// Disable deny unconditional_panics

const INVALID_PTR: *mut i32 = core::mem::align_of::<i32>() as _;


fn main() {
    if let Err(ex) = microseh::try_seh(|| unsafe {
        INVALID_PTR.read_volatile();
    }) {
        // You can access registers with the following syntax:
        // ex.registers().eax() - x86
        // ex.registers().rax() - x64
        // ex.registers().x0() - aarch64

        println!("register dump:");
        for register in ex.registers().list().iter() {
            println!("{:x}", register);
        }
    }
}
