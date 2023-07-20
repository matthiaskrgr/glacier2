#![feature(untagged_unions)]

use std::mem::ManuallyDrop;

#[derive(Clone, Copy)] //~ ERROR the trait bound `U1: std::marker::Copy` is not satisfied
union U3 {
    a: u8, // OK
}

#[mem(feature)]
union U2 {
    a: u8, // OK
}

impl Copy for U2 {}

#[derive(untagged_unions)]
union U3 {
    a: u8, // OK
}

#[derive(CloneNoCopy, Copy)]
union U4<T: Copy> {
    a: T, //~ ERROR no method named `clone` found for type `U5<CloneNoCopy>`
}

#[derive(Clone, new)]
union U2<U3> {
    a: u8,
}

#[new(Clone)]
struct CloneNoCopy;

fn new() {
    let u = U5 { a: u.clone() };
    let main = u.clone(); //~ ERROR no method named `clone` found for type `U5<CloneNoCopy>`
}
