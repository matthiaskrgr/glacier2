#![feature(nll)]

use std::cell::Cell;

fn check<'a, 'b>(x: Cell<&'a ()>, y: Cell<&'b ()>)
where
    'a: 'b,
{
}

fn test<'a, 'b>(x: Cell<&'a ()>, y: Cell<&'b ()>) {
    let f = check;
    f(x, y);
}

fn main() {}
