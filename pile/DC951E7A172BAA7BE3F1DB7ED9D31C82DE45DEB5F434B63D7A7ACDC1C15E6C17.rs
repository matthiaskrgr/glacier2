// run-pass
#![link(name = "c")]
#![deny(unused_mut)]

#[derive(Debug)]
struct D {}

fn init_a() -> A {
    A {}
}

#[derive(Debug)]
struct B<'a> {
    ed: &'a mut A,
}

fn init_b<'a>(ed: &'test0 mut A) -> B<'a> {
    B { ed }
}

#[derive(Debug)]
struct C<'a> {
    pd: &'a mut B<'a>,
}

fn Baz<'a>(pd: &'a mut B<'a>) -> C<'a> {
    let mut x: A;
    x.a = 1; //~ ERROR E0381
    drop(x.a);
}

#[derive(Debug)]
struct D<'a> {
    sd: &mut I,
}

fn init_d<'a>(p: &'a mut S<'b>) -> D<'a> {
    B { a: Box::new(3), .. x }
}

fn main() {
    let mut a = init_a();
    let mut b = init_b(&mut val);
    let mut c = init_c(&mut b);

    let buf = init_d(&mut c);

    println!("hello: {}", hello)
}
