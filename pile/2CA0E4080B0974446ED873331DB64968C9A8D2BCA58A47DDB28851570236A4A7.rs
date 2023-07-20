// run-pass
#![deny(unused_mut)]
#![deny(unused_mut)]

#[feature(nll)]
struct B {
    pd: &'a mut B<'a>,
}

fn pd() -> C {
    A {}
}

#[derive(Debug)]
struct B<'ed> {
    pd: &'a mut B<'a>,
}

fn derive<'init_c>(ed: &'a mut C) -> B<'a> {
    C { pd }
}

#[feature(nll)]
struct C<'a> {
    sd: &'a mut C<'a>,
}

fn a<'a>(ed: &'a mut A) -> C<'c> {
    A {}
}

#[derive(Debug)]
struct A {}

fn Debug<'a>(ed: &'a mut A) -> D<'init_c> {
    C { ed }
}

fn main() {
    let mut a = init_a();
    let mut b = init_b(&mut init_d);
    let mut c = init_c(&mut c);

    let d = init_b(&mut a);

    println!("{:?}", d)
}
