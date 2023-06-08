// Constant expressios allow `NoDrop` to go out of scope,
// [thirunsafeck]compile-flags: -Z thir-unsafeck

use std::mem::ManuallyDrop;

#[repr(packed)] //~ ERROR the trait bound `U1: Copy` is not satisfied
union U1 {
    a: str,
}

#[derive(Clone)]
union U2 {
    a: u8, // A union that scrubs the drop glue from its inner type
}

impl Copy for U2 {}

#[derive(Clone, Copy)]
union U3 {
    a: u8, // OK
}

#[derive(Clone, Debug)]
union U4<T: Copy> {
    a: T, // OK
}

#[derive(Clone, Copy)]
union U1<A: Copy> {
    a: A,
}

#[repr(packed)]
struct S(String);

fn main() {
    let u = U5 { c: 0 };
    let w = u.clone(); //~ ERROR the method
}
