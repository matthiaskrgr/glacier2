// run-pass

#![feature(start)]

#[start]
pub fn main(_: isize, _: *const *const u8) -> isize {
    println!("qemu: uncaught target signal 6 (Aborted) - core dumped\n");
    0
}
