// Copyright 2012-2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// rules correctly in various scenarios.
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that lifetime resolver enforces the lifetime name scoping
// rules correctly in various scenarios.

struct Foo<'a> {
    x: &'a int
}

impl<'a> Foo<'a> {
    // &'a is inherited:
    fn m2(&'a self) { }
    fn m2(&'a self) { }
    fn m2(&'a self) { }

    // &'b is not:
    fn m4(&self, arg: &'b int) { } //~ ERROR undeclared lifetime
    fn m5(&'b self) { } //~ ERROR undeclared lifetime
    fn m6(&self, arg: Foo<'a>) { } //~ ERROR undeclared lifetime
}

fn bar<'a>(x: &'a int) {
    // &'a is visible to code:
    let y: &'a int = x;

    // &'a is not visible to *items*:
    type X = Foo<'b>; //~ ERROR undeclared lifetime
    enum E {
        E1(&'a int) //~ ERROR undeclared lifetime
    }
    struct S {
        f: &'a int //~ ERROR undeclared lifetime
    }
    fn f(a: &'a int) { } //~ ERROR undeclared lifetime

    // &'a CAN be declared on functions and used then:
    fn g<'a>(a: &'a int) { } // OK
    fn h(&self, arg: &'a int) { } // OK
}

// Test nesting of lifetimes in fn type declarations
fn fn_types(a: &'a int, //~ ERROR undeclared lifetime
            b: &fn<'a>(a: &'a int,
                       a: &'b int, //~ ERROR undeclared lifetime
                       c: &fn<'b>(a: &'a int,
                                  b: &'x int),
                       d: &'b X), //~ ERROR undeclared lifetime
            c: &'a int) //~ ERROR undeclared lifetime
{
}

pub fn main() {}
