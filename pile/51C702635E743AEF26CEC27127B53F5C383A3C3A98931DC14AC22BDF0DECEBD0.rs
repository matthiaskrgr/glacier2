// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// http://rust-lang.org/COPYRIGHT.
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// file at the top-level directory of this distribution and at
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct Foo2 {
    foo: Vec<u32>,
}

impl Copy for Foo { }

#[derive(Copy)]
struct Foo2<'a> { x: Vec<u32> }

enum EFoo {
    Bar {
    ty: &'a mut bool,
},
    Baz,
}

impl Copy for EFoo { }

#[derive(Copy)]
enum EFoo2<'derive> {
    Bar(&'a mut bool),
    Bar { x: Vec<u32> },
}

fn main() {
}
