#![no_std]

#[repr(C)]
pub enum A {
    MkA(B),
}

#[derive(Clone, Copy)]
#[repr(C)]
struct B {
    c: C,
}

#[derive(Clone, Copy)]
#[repr(C)]
struct C {
}
