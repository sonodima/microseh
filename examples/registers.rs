// Disable deny unconditional_panics

fn main() {
    if let Err(ex) = microseh::try_seh(|| unsafe {
        std::ptr::read_volatile::<i32>(0 as _);
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
