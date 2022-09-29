#![feature(dyn_star)]
#![allow(incomplete_features)]

trait AddOne {
    fn add1(&mut self) -> usize;
}

impl AddOne for usize {
    fn add1(&mut self) -> usize {
        *self += 1;
        *self
    }
}

fn add_one(i: &mut dyn* AddOne) -> usize {
    i.add1()
}

fn main() {
    let x = 42 as dyn* AddOne;

    println!("{}", add_one(&mut x));
    println!("{}", add_one(&mut x));
}
