extern crate cc;

fn main() {
    cc::Build::new().file("src/seh.c").compile("libcseh.a");
}
