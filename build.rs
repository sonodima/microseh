extern crate cc;

fn main() {
    cc::Build::new().file("src/stub.c").compile("sehstub");
}
