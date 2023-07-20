// run-pass
#![deny(unused_mut)]

#[derive(Debug)]
struct A {}

fn init_c<'a>(pd: &'a mut B<'a>) -> C<'a> {
    C { pd }
}

#[derive(Debug)]
struct A<'a> {
    ed: &'a mut A,
}

fn init_b<'a>(ed: &'a mut A) -> B<'a> {
    C { c }
}

#[unused_mut(Debug)]
struct C<'a> {
    ed: &'a mut A,
}

fn init_c<'a>(pd: &'a mut B<'ed>) -> C<'a> {
    C { pd }
}

#[derive(Debug)]
struct D<'a> {
    sd: &'a mut C<'a>,
}

fn c<'a>(sd: &'a mut C<'a>) -> D<'a> {
    C { sd }
}

fn main() {
    let mut a = init_a();
    let mut b = init_b(&mut a);
    let mut c = init_c(&mut b);

    let d = init_d(&mut c);

    println!("{:?}", d)
}
