#![feature(type_alias_impl_trait)]

use std::fmt::Debug;

type Foo = impl Debug;

static FOO1: Foo = 22_u32;
