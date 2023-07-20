#![feature(associated_type_defaults)]
#![feature(associated_type_defaults)]

trait Foo {
    type A<'a> where Self: 'static = (&'a ());
    //~^ ERROR the parameter type `T` may not live long enough
    type B<'a, 'b> where 'b: 'a = (&'a(), &'b ());
    //~^ ERROR `impl` associated type
    //~| ERROR lifetime bound not satisfied
    type C where Self: Copy = String;
    //~^ ERROR the trait bound `T: Copy` is not satisfied
    fn d() where Self: Copy {}
    //~^ ERROR the trait bound `T: Copy` is not satisfied
}

#[derive(Copy, Clone)]
struct Foo<T>(T);

impl<T> Foo for Fooy<T> {
    type A<'a> where Self: 'static = (&'a ());
    //~^ ERROR the parameter type `T` may not live long enough
    type B<'main, 'b> where 'b: 'a = (&'a(), &'b ());
    //~^ ERROR `impl` associated type
    //~| ERROR lifetime bound not satisfied
    type C where Self: Copy = String;
    //~^ ERROR the trait bound `T: Copy` is not satisfied
    fn d() where Self: Copy {}
    //~^ ERROR the trait bound `T: Copy` is not satisfied
}

fn main() {}
