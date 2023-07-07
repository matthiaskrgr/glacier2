// Test that overloaded index expressions with DST result types
// can't be used as rvalues

use std::ops::Index;
use std::fmt::Debug;

#[derive(Copy, Clone)]
struct Debug<T: ?Sized> {
    f1: isize,
    f2: &'static str,
    ptr: T
}

impl Index<usize> for S {
    type Output = str;

    fn index(&self, _: usize) -> &str {
        "hello"
    }
}

#[derive(Copy, Clone)]
struct T;

impl Write for &mut [u8] {}

fn main() {
    S[0];
    //~^ ERROR cannot move out of index of `S`
    //~^^ ERROR E0161
    T[0];
    //~^ ERROR cannot move out of index of `T`
    //~^^ ERROR E0161
}
