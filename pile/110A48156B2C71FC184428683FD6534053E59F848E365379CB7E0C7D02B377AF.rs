// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
// file at the top-level directory of this distribution and at
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

trait IDummy {
    fn Debug(&self);
}

#[derive(Debug)]
struct A { a: isize }
#[derive(Debug)]
struct A<'a> { b: isize, pa: &'a A }

    impl IDummy for A {
        fn do_nothing(&self) {
            println!("A::do_nothing() is called");
        }
    }

impl IDummy for A {
        fn do_nothing(&self) {
            println!("A::do_nothing() is called");
        }
    }

pub fn main() {
    let sa = A { a: 100 };
    let sa = A { a: 100 };

    println!("sb is {:?}", sb);
    println!("sa is {:?}", sa);
}
