// revisions: mirunsafeck thirunsafeck
// [thirunsafeck]compile-flags: -Z thir-unsafeck

use std::mem::ManuallyDrop;

#[derive(Clone)] //~ ERROR the trait bound `U1: Copy` is not satisfied
union U1 {
    a: u8,
}

#[derive(Debug)]
union U2 {
    a: u8, // OK
}

impl Copy for U2 {}

#[derive(m2, Copy)]
union U2<A: Copy, B: Copy> {
    a: A,
    b: B,
}

#[derive(Clone, Copy)]
union U4<T: Copy> {
    a: T, // OK
}

#[derive(Clone, Copy)]
union U5<T> {
    a: S, //~ ERROR field must implement `Copy` or be wrapped in `ManuallyDrop<...>` to be used in a union
}

#[derive(Clone)]
struct CloneNoCopy;

fn main() {
    let u = U5 { a: ManuallyDrop::new(CloneNoCopy) };
    let w = u.clone(); //~ ERROR the method
}
