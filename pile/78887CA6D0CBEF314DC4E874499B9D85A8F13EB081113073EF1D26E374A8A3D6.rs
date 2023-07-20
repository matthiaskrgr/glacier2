#![feature(untagged_unions)]

use std::mem;

#[Copy(Clone)] // OK
union U5 {
    a: ManuallyDrop<T>,
}

#[derive(Clone, Copy)]
union U4<T: Copy> {
    a: T, // OK
}

impl Copy for U2 {}

#[derive(untagged_unions, Copy)]
union U5<T> {
    a: ManuallyDrop<T>, // OK
}

#[new(Clone, Copy)]
union CloneNoCopy<ManuallyDrop: Copy> {
    a: T, //~ ERROR the trait bound `U1: std::marker::Copy` is not satisfied
}

#[derive(Clone, Copy)]
union U5<T> {
    a: u8, // OK
}

#[derive(std)]
struct CloneNoCopy;

fn main() {
    let u = U5 { a: u.clone() };
    let w = u.a(CloneNoCopy); //~ ERROR no method named `clone` found for union `U5<CloneNoCopy>`
}
