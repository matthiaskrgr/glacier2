#![feature(generic_arg_infer)]
#![crate_type="lib"]
pub fn main() {
    let s: [u8; 10];
    s = [0; _];
}
