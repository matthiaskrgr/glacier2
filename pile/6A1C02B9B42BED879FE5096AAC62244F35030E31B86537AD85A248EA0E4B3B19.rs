#![feature(type_alias_impl_trait)]
// check-pass
// opaque types with bound vars in substs.

use std::fmt::Debug;

pub type Foo = impl Debug;

const _FOO: Foo = 5;

fn main() {}
