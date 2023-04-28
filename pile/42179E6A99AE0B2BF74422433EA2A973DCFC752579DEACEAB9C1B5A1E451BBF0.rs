// check-pass
#![allow(dead_code)]
// check-pass

#[allow(dead_code)]

use std::marker::PhantomData;

fn f<'a>(PhantomData::<&'a u8>: PhantomData<&'a u8>) {}

fn main() {}
