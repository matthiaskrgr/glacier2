#![feature(raw_dylib)]
#[link(name = "kernel32", kind = "raw-dylib")]
extern "system" {
    fn WaitForSingleObject(handle: isize, dwmilliseconds: u32) -> u32;
}
fn main() {
    unsafe { WaitForSingleObject(0, 0); }
    println!("ok");
}
