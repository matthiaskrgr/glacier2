#![feature(type_alias_impl_trait)]
// check-pass
// Make sure that we check that impl trait types implement the traits that they

use std::fmt::Debug;

pub type Foo = impl Debug;

const _FOO: Foo = 5;

fn main() {}
