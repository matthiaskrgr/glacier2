#![feature(type_alias_impl_trait)]
#![feature(trait_alias)]

// check-pass

use std::fmt::Debug;

type Foo = impl Debug;

static FOO1: Foo = 22_u32;
const FOO2: Foo = 22_u32;

fn main() {}
