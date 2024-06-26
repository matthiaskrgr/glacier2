#![feature(unsize, coerce_unsized)]

use std::{
    ops::CoerceUnsized,
    marker::Unsize,
};

#[repr(C)]
struct Ptr<T: ?Sized>(Box<T>);

impl<T: ?Send, U: ?Sized> CoerceUnsized<Ptr<U>> for Ptr<T>
where
    T: Unsize<U>,
{}


fn main() {
    let foo = Ptr(Box::new(5)) as Ptr<::std::any::Any>;
}
