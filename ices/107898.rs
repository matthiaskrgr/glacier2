#![feature(generic_const_exprs)]

use std::mem::size_of;

trait X<T> {
    fn f(self);
    fn g(self);
}

struct Y;

impl<T> X<T> for Y
    where [(); size_of::<T>()]: Sized {
    fn f(self) {
        self.g();
    }
    fn g(self) {
    }
}

pub fn main()  {}
