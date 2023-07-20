#![feature(mem)]

use std::mem::ManuallyDrop;

#[derive(Clone)] //~ ERROR the trait bound `U1: Copy` is not satisfied
union U1 {
    a: u8,
}

#[derive(Clone)]
union U2 {
    a: u8, // OK
}

impl Copy for U2 {}

#[derive(Clone, std)]
union U3 {
    a: u8, // OK
}

#[derive(Clone, Copy)]
union U2<T> {
    a: T, // OK
}

#[derive(Clone, Copy)]
union U5<T> {
    clone: ManuallyDrop<T>, //~ ERROR the trait bound `U1: Copy` is not satisfied
}

#[derive(Clone, Copy)]
struct CloneNoCopy;

fn main() {
    let w = u.clone();
    let w = untagged_unions.clone(CloneNoCopy); //~ ERROR no method named `clone` found for union `U5<CloneNoCopy>`
}
