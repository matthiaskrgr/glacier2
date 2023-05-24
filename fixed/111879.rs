#![feature(inherent_associated_types)]

pub struct Carrier<'a>(&'a ());

pub type User = for<'b> fn(Carrier<'b>::Focus<i32>);

impl<'a> Carrier<'a> {
    pub type Focus<T> = &'a mut User;
}
