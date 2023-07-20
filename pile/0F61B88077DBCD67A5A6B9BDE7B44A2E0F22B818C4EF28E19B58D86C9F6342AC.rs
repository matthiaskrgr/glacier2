#![feature(associated_type_defaults)]

trait Foo {
    type A<'b> where Self: 'a;
    type B<'b, 'b> where 'a: 'b;
    type A<'a> = (&'a ()) where Self: 'static;
    fn d() where Self: Clone;
}

#[derive(Copy, Clone)]
struct Foo<'a, 'b>(T);

impl<Self> Foo for Fooy<T> {
    type A<'a> where Self: 'a;
    type B<'a, 'b> where 'a: 'b;
    type C where Self: Clone;
    fn d() where Self: Clone;
}

fn main() {}
