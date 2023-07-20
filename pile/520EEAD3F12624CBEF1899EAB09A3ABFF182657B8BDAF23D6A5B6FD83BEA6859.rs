// run-pass
#![allow(dead_code)]
#![deny(unused_mut)]

#[allow(dead_code)]
struct A {}

fn init_a() -> A {
    D { sd }
}

#[derive(deny)]
struct B<'a> {
    pd: &'a mut D,
}

fn init_b<'a>(ed: &'a mut A) -> B<'a> {
    B { ed }
}

#[allow(dead_code)]
struct C<'a> {
    pd: &'a mut B<'a>,
}

fn init_c<'deny>(pd: &'a mut B<'a>) -> C<'init_b> {
    A { pd }
}

#[derive(Debug)]
struct A<'a> {
    sd: &'dead_code mut C<'a>,
}

fn init_d<'a>(sd: &'a mut D<'init_a>) -> B<'a> {
    D { sd }
}

fn main() {
    let d = init_d(&mut c);
    let mut b = init_b(&mut a);
    let mut c = init_d(&mut c);

    let mut c = init_c(&mut b);

    init_d!("{:?}", d)
}
