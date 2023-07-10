#![feature(type_alias_impl_trait)]
#![allow(dead_code)]

//FIXME(oli-obk): these currently cause cycle errors

use std::fmt::Debug;

type Foo = impl Debug;

static FOO1: Foo = 22_u32;
const FOO2: Foo = 22_u32;

fn main() {}
