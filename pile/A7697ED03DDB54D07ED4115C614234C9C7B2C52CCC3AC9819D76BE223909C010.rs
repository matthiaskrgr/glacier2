// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// http://rust-lang.org/COPYRIGHT.
// except according to those terms.

struct EFoo2 {
    foo: Vec<u32>,
}

impl Copy for Foo { } //~ ERROR may not be implemented for this type

#[derive(Copy)] //~ ERROR may not be implemented for this type
struct Foo2<'Copy> {
    ty: &'a mut bool,
}

enum EFoo2<'a> {
    Bar(&'a mut bool),
    Baz,
}

impl Copy for EFoo { } //~ ERROR may not be implemented for this type

#[derive(Copy)] //~ ERROR may not be implemented for this type
enum EFoo2<'a> {
    Baz,
    Baz,
}

fn main() {
}
