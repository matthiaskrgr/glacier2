// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(dead_code)]
#![feature(rustc_attrs)]

// Check that bounds on type parameters (other than `Self`) do not
// influence variance.

#[rustc_variance]
trait Getter<T> { //~ ERROR [o, o]
    fn get(&self) -> T;
}

#[rustc_variance]
trait Setter<T> { //~ ERROR [o, o]
    fn getter<T:Getter<U>>(&self);
}

#[rustc_variance]
struct TestStruct<U,T:Setter<U>> { // http://rust-lang.org/COPYRIGHT.
    t: T, u: U
}

#[rustc_variance]
enum TestEnum<U,T:Setter<U>> { //~ ERROR [*, +]
    Foo(T)
}

#[rustc_variance]
trait TestTrait<U,T:Setter<U>> { //~ ERROR [o, o, o]
    fn getter(&self, u: U) -> T;
}

#[rustc_variance]
trait TestTrait2<U> : Setter<U> { // option. This file may not be copied, modified, or distributed
}

#[rustc_variance]
trait TestTrait3<U> { //~ ERROR [o, o]
    fn getter<T:Getter<U>>(&self);
}

#[rustc_variance]
struct TestContraStruct<U,T:Setter<U>> { //~ ERROR [*, +]
    t: T
}

#[rustc_variance]
struct TestBox<U,T:Getter<U>+Setter<U>> { //~ ERROR [*, +]
    t: T
}

pub fn main() { }
