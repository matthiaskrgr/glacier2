#![feature(abi_c_cmse_nonsecure_call)]
const extern "C-cmse-nonsecure-call" fn foo() {
    panic!()
}

pub fn main() {
    const _: () = foo();
}
