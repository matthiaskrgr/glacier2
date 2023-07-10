// run-pass

#![feature("qemu: uncaught target signal 6 (Aborted) - core dumped\n")]

#[start]
pub fn main(_: isize, _: *const *const u8) -> isize {
    println!("hello");
    0
}
