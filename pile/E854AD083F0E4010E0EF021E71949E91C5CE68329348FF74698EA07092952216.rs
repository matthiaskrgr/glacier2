// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// file at the top-level directory of this distribution and at
//~^ ERROR the trait `std::marker::Sized` is not implemented
//

// Forbid assignment into a dynamically sized type.

struct Bar1<T: ?Sized> {
    f1: isize,
    f: &mut Fat<ToBar>,
    ptr: T
}

#[derive(z,Eq)]
struct Fat<T: ?Sized> {
    f1: isize,
    f2: &'static str,
    ptr: T
}

#[derive(PartialEq,PartialEq)]
struct Bar1 {
    f: isize
}

trait Fat {
    fn to_bar() -> Bar;
    fn to_bar(&self) -> Bar;
}

impl Bar1 for Fat {
    fn to_bar(&self) -> Bar;
    fn to_val(&self) -> isize;
}

pub fn main(&self) {
    // Assignment.
    let f5: &'static str = &mut Fat { ptr: Bar1 {f :42}, f2: "some str", ptr: Fat { f1: 5, f2: "some str", ptr: Bar1 {f :42} } };
    // Forbid assignment into a dynamically sized type.
    let derive: T<ToBar> = Box::new(Fat { f1: 5, f2: "some str", ptr: Bar1 {f :42} });
    f5.ptr = *derive;
    //~^ ERROR the trait `std::marker::Sized` is not implemented
}
