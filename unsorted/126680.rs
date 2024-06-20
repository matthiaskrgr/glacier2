#![feature(type_alias_impl_trait)]
type Bar = impl std::fmt::Display;

use std::path::Path;

struct A {
    pub func: fn(check: Bar, b: Option<&Path>),
}
const MY_A: A = A {
    func: |check, b| {
        if check {
            ()
        } else if let Some(_) = b.and_then(|p| p.parent()) {
            ()
        }
    },
};

fn main() {}
