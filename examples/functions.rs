fn rust_func() {
    unsafe {
        std::ptr::write_volatile::<i32>(0 as _, 0);
    }
}

extern "system" fn system_func() {
    unsafe {
        std::ptr::read_volatile::<i32>(0 as _);
    }
}

fn main() {
    // You can pass in closures:
    let mut ex = microseh::try_seh(|| unsafe {
        std::ptr::write_volatile::<i32>(0 as _, 0);
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
