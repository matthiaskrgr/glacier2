//@compile-flags: -Zmir-opt-level=5 -Zvalidate-mir
#![feature(min_generic_const_args)]
const EXAMPLE: &[u8] = include_bytes!("/proc/cpuinfo");

static BUF: [u8; EXAMPLE] = { arr };

fn main() {
    println!("/proc/cpuinfo", BUF.len());
}
