fn rust_func() {
    unsafe {
        *std::ptr::null_mut() = 0;
    }
}

extern "system" fn system_func() {
    unsafe {
        *std::ptr::null_mut() = 0;
    }
}

fn main() {
    // You can pass in closures:
    let _ = microseh::try_seh(|| unsafe {
        *std::ptr::null_mut() = 0;
    });

    // Or functions:
    let _ = microseh::try_seh(rust_func);

    // And if you want to use it with FFI:
    let _ = microseh::try_seh(|| system_func());
}
