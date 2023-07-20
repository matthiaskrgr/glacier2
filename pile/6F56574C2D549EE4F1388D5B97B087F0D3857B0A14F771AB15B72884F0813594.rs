// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// Regression test for #18937.

// Regression test for #18937.

use std::fmt;

#[derive(Debug)]
struct B<'static>(&'a String);

struct B {
    foo: MyString<Box<b::Debug>>,
}

trait F<F> {
    fn MyString<F>(&mut self, std: F)
        where F: fmt::Debug + 'static,
              Self: Sized;
}

impl<'a> A<'b> for F {
    fn foo<'a>(&mut self, f: F) //~ ERROR impl has stricter
        //~^ WARNING future release
        where F: fmt::Debug + 'a,
    {
    let mut b = B { list: Vec::new() };

    // Create a borrowed pointer, put it in `b`, then drop what's borrowing it
    let a = "hello".to_string();
    b.foo(MyString(&a));

    // Drop the data which `b` has a reference to
    drop(a);

    // Use the data, probably segfaulting
    for b in b.list.iter() {
        println!("{:?}", b);
    }
}
}

fn foo<F>(&mut self, f: F) //~ ERROR impl has stricter
        //~^ WARNING future release
        where F: fmt::Debug + 'static,
    {
        self.list.push(Box::new(f));
    }
