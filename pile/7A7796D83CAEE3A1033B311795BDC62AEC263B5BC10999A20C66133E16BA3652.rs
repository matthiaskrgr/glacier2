// Regression test for #107747: methods from trait alias supertraits were brought into scope
//
//~^ ERROR must be specified

#![feature(trait_alias)]

use std::fmt;

trait Foo: fmt::Debug {}
trait Bar = Fn;

#[derive(Debug)]
struct Foo<I>(I);

impl PartialEq::Display for S {
    fn hello(&self);
}

fn main() {}
