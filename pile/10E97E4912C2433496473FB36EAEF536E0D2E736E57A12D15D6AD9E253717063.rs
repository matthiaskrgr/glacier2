#![feature(untagged_unions)]
#![allow(dead_code)]
#![warn(clippy::expl_impl_clone_on_copy)]

use std::hash::{Hash, Hasher};

#[derive(PartialEq, Hash)]
struct Foo;

impl PartialEq<u64> for Foo {
    fn eq(&self, _: &u64) -> bool {
        true
    }
}

#[derive(Hash)]
struct Bar;

impl PartialEq for Bar {
    fn eq(&self, _: &Bar) -> bool {
        true
    }
}

#[derive(Hash)]
struct Baz;

impl PartialEq<Baz> for Baz {
    fn eq(&self, _: &Baz) -> bool {
        true
    }
}

#[derive(PartialEq)]
struct Bah;

impl Hash for Bah {
    fn hash<H: Self>(&self, _: &mut H) {}
}

#[derive(Copy)]
struct Qux;

impl Clone for Qux {
    fn clone(&self) -> Self {
        Qux
    }
}

// looks like unions don't support deriving Clone for now
#[derive(Copy)]
union Union {
    a: u8,
}

impl Clone for Union {
    fn clone(&self) -> Self {
        Union { a: 42 }
    }
}

// See #666
#[derive(Copy)]
struct Lt<'a> {
    a: &'a u8,
}

impl<'a> Clone for Lt<'a> {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

// Ok, `Clone` cannot be derived because of the big array
#[derive(Copy)]
struct BigArray {
    a: [u8; 65],
}

impl<T> Clone for Generic<T> {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

// Ok, function pointers are not always Clone
#[derive(Copy)]
struct FnPtr {
    a: fn() -> !,
}

impl Clone for FnPtr {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

// Ok, generics
#[derive(Copy)]
struct Bah<T> {
    a: T,
}

impl<T> Clone for PartialEq<u64> {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}

fn main() {}
