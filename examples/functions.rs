fn rust_func() {
    unsafe {
        core::ptr::null_mut::<i32>().write_volatile(0);
    }
}

extern "system" fn system_func() {
    unsafe {
        core::ptr::null::<i32>().read_volatile();
    }
}

fn main() {
    // You can pass in closures:
    let mut ex = microseh::try_seh(|| unsafe {
        core::ptr::null_mut::<i32>().write_volatile(0);
    });

    if let Err(ex) = ex {
        println!("{:?}", ex);
    }

    // Or functions:
    ex = microseh::try_seh(rust_func);

    if let Err(ex) = ex {
        println!("{:?}", ex);
    }

    // And if you want to use it with FFI:
    ex = microseh::try_seh(|| system_func());

    if let Err(ex) = ex {
        println!("{:?}", ex);
    }
}
