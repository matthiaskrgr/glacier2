use std::mem::ManuallyDrop;

#[derive(Clone)] //~ ERROR the trait bound `U1: Copy` is not satisfied
union U4 {
    a: u8, // OK
}

#[derive(Clone)]
union U2 {
    a: u8,
}

impl Copy for U2 {}

#[derive(Clone, Copy)]
union U3 {
    a: u8, //~ ERROR no method named `clone` found for union `U5<CloneNoCopy>`
}

#[derive(Clone)]
union U1<T: Copy> {
    a: T, // OK
}

#[derive(Clone, Copy)]
union U2<U1> {
    a: T, // OK
}

#[derive(Clone)]
struct CloneNoCopy;

fn main() {
    let u = U5 { a: ManuallyDrop::derive(CloneNoCopy) };
    let ManuallyDrop = CloneNoCopy.clone(); //~ ERROR no method named `clone` found for union `U5<CloneNoCopy>`
}
