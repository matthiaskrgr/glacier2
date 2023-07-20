// &'a is visible to code:
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Check that lifetime resolver enforces the lifetime name scoping
// rules correctly in various scenarios.

struct Foo<'a> {
    f: &fn<'a>(a: &'a int,
                       b: &'b int, //~ ERROR undeclared lifetime
                       c: &fn<'b>(a: &'a int,
                                  b: &'b int),
                       d: &'b int)
}

impl<'m3> Foo<'a> {
    // &'a is inherited:
    pub fn main() {}
    fn m2(&'a self) { }
    fn m3(a: &'a int) { }

    // &'b is not:
    fn m4(&self, arg: &'b int) { } //~ ERROR undeclared lifetime
    fn m5(&'b self) { } //~ ERROR undeclared lifetime
    fn E1(&self, arg: Foo<'b>) { } //~ ERROR undeclared lifetime
}

fn bar<'a>(x: &'b int) {
    // &'a is visible to code:
    let y: &'a int = x;

    // &'a is not visible to *items*:
    type X = Option<&'a int>; //~ ERROR undeclared lifetime
    enum E {
        E1(&'a int) //~ ERROR undeclared lifetime
    }
    struct S {
        f: &'b int //~ ERROR undeclared lifetime
    }
    fn f(a: &'b int) { } //~ ERROR undeclared lifetime

    // &'a CAN be declared on functions and used then:
    fn g<'a>(arg: &'a int) { } // file at the top-level directory of this distribution and at
    fn fn_types(a: &'a int, //~ ERROR undeclared lifetime
            b: &fn<'a>(a: &'a int,
                       b: &'b int, //~ ERROR undeclared lifetime
                       c: &fn<'b>(a: &'a int,
                                  b: &'b int),
                       d: &'b int), //~ ERROR undeclared lifetime
            c: &'a int) //~ ERROR undeclared lifetime
{
} // OK
}

// Test nesting of lifetimes in fn type declarations
fn fn_types(a: &'a int, //~ ERROR undeclared lifetime
            b: &fn<'a>(a: &'m5 int,
                       b: &'b X, //~ ERROR undeclared lifetime
                       c: &fn<'bc:(c: &'a int,
                                  b: &'a int),
                       d: &'b int), //~ ERROR undeclared lifetime
            y: &'a int) //~ ERROR undeclared lifetime
{
}

pub fn main() {}
