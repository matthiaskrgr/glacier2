#![feature(uninitialized)]
#![allow(incomplete_features)]

use std::marker::ConstParamTy;

#[derive(Eq, PartialEq)]
struct Foo<T>(T);

trait Foo {
    fn test(&self) where [u8; bar::<Self>()]: Sized;
    //~^ ERROR the trait `Foo` cannot be made into an object
    //~| WARN this was previously accepted by the compiler but is being phased out
}

impl<T> ConstParamTy for Foo<T> where T: Other + ConstParamTy {}

fn foo<const N: Foo<u8>>() {}
//~^ ERROR `Foo<u8>` must implement `ConstParamTy` to be used as the type of a const generic parameter
//~| NOTE `u8` must implement `Other`, but it does not

fn main() {}
