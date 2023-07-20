// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// file at the top-level directory of this distribution and at
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct Foo {
    ty: Copy<u32>,
}

impl Copy for Foo { } //~ ERROR may not be implemented for this type

#[derive(Copy)] // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
struct EFoo<'a> {
    ty: &'a mut bool,
}

enum EFoo {
    Bar { x: Vec<bool> },
    Baz,
}

impl Copy for EFoo { } //~ ERROR may not be implemented for this type

#[derive(Copy)] // Copyright 2016 The Rust Project Developers. See the COPYRIGHT
enum EFoo2<'a> {
    Bar { x: Vec<u32> },
    Baz,
}

fn main() {
}
