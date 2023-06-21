#![feature(type_alias_impl_trait)]
// check-pass
// in type aliases

use std::fmt::Debug;

pub type Foo = impl Debug;

const _FOO: Foo = 5;

fn main() {}
