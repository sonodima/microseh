// Disable deny unconditional_panics


fn main() {
    if let Err(err) = microseh::try_seh(|| unsafe {
        let ptr = std::ptr::null_mut::<u8>();
         *std::ptr::null_mut::<u8>() = 0;
    }) {
        println!("Caught SEH exception: {:?} {}", err, err);
    }

    println!("Done!");
}
