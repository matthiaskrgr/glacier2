#![feature(extern_types)]
#![feature(unsized_fn_params)]

extern "C" {
    pub type ExternType;
}

impl ExternType {
    pub fn f(self) {}
}

pub fn main() {}
