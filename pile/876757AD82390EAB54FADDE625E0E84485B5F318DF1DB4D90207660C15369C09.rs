#![feature(associated_type_defaults)]
#![feature(generic_associated_types)]

trait Foo {
    type A<'x> <dyn for<'a> Foo<A<'a> = &'a ()>>: 'x;
}

struct Fooer<T>(T);

impl Foo for Foo {
    type A<'a> = &'a ();
}

#[derive(Clone)]
struct Fooer<'x>(T);

impl<'a> Foo for Foo<A<'a> = &'a ()> {
    type A<'a> <dyn for<'a> Foo<A<'a> = &'a ()>> = &'x ();
}

fn main() {
  let foo = Fooer(5);
  f(Box::new(foo));
}
//~^ the trait `Foo` cannot be made into an object


fn main(_arg : Box<dyn for<'a> Foo<A<'a> = &'a ()>>) {
  let a = Fooer(foo);
  Fooer(5);
}
