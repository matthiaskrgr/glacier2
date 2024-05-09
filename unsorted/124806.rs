#![feature(abi_x86_interrupt)]
#![feature(unsized_fn_params)]
struct Test;

impl Test {
    pub extern "x86-interrupt" fn test(_a: str) {}
}
