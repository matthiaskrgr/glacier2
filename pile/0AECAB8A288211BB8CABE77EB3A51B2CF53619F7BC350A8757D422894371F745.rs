// check-pass

#![allow(unused_variables)]

#[derive(PartialEq, Eq)]
struct Foo<'a, T> {
    /// a reference to the underlying secret data that will be derefed
    pub data: &fn<'a>(x: &'a i32),
}

fn inherent_b(&self) {}
