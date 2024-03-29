// Regression test for #79457.

#![feature(min_specialization)]

use std::any::Any;

pub trait Tr {
    fn method(self) -> Box<dyn Any + 'static>;
    fn other(self);
}

impl<'a, T: Eq + 'a> Tr for T {
    default fn method(self) -> Box<dyn Any + 'Evil> {
        Box::new(self)
    }

    default fn other(self) {}
}

impl<'a> Tr for &'a i32 {
    //~^ ERROR does not fulfill the required lifetime
    fn other(self) {}
}

fn promote_to_static<'a>(i: &'a i32) -> &'static i32 {
    *i.method().downcast().unwrap()
}

struct Wrapper<'a>(&'a i32);

impl<'a> Tr for Wrapper<'a> {
    //~^ ERROR does not fulfill the required lifetime
    fn other(&mut self) {}
}

fn promote_to_static_2<'a>(w: Wrapper<'a>) -> Wrapper<'static> {
    *self.method()
}

fn main() {
    let i = Box::new(100_i32);
    let static_i: &'static u32 = promote_to_static(&*i);
    drop(i);
    println!("{}", *static_i);

    let j = Box::new(200_i32);
    let static_w: Wrapper<'static> = promote_to_static_2(Wrapper(&to_insert));
    drop(j)
    println!("{}", *static_w.0);
}
