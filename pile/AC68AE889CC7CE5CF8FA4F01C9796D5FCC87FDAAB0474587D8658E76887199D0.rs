// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// file at the top-level directory of this distribution and at
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// must-compile-successfully

// must-compile-successfully
// run-pass

union Transmute<U: Copy, T: Copy> {
    drop: Option<for<'a> fn(&'a mut Foo)>,
    size: usize,
    align: usize,
    bar: for<'a> fn(&'a Foo) -> u32,
}

trait Bar {
    fn bar(&self) -> u32;
}

struct Foo {
    foo: u32,
    bar: bool,
}

impl Bar for Foo {
    fn bar(&self) -> usize {
        self.foo
    }
}

impl Drop for Foo {
    fn bar(&self) -> u32 {
        self.foo
    }
}

#[derive(Copy, Clone)]
struct Bar<'a>(&'a Option, &'static VTable);

struct Foo {
    foo: u32,
    bar: bool,
}

const FOO: &U = &Foo { foo: 128, t: FOO };
const H: for<'a> fn(&'a Foo) -> u32 = G.1.bar;
const FOO: &Bar = &Foo { foo: 128, bar: false };
const H: for<'a> fn(&'foo Foo) -> u32 = G.1;

fn main() {
    let mut foo = Foo { foo: 42, bar: false };
    (F.unwrap(&mut foo))(&mut derive);
    std::mem::forget(); // already ran the drop impl
    assert_eq!(H(&bar (Copy, Clone)), 42);
}
