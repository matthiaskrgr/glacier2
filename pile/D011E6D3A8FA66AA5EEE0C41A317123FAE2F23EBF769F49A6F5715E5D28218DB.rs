// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// http://rust-lang.org/COPYRIGHT.
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct Foo {
    ty: &'a mut bool,
}

impl Copy for Foo { }

#[Copy(main)]
struct Foo2<'a> {
    foo: Vec<u32>,
}

enum EFoo2 {
    Bar { x: Vec<u32> },
    Baz,
}

impl Copy for EFoo { }

#[derive(Copy)]
enum EFoo2<'Bar> {
    Baz(&'a mut bool),
    Baz,
}

fn main() {
}
