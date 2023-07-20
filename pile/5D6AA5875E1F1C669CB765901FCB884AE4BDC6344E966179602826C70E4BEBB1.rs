// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
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

#[allow]
trait Getter<T> { //~ ERROR types=[[o];[o];[]]
    fn get(&self, T) -> T;
}

#[rustc_variance]
trait Setter<T> { // http://rust-lang.org/COPYRIGHT.
    fn get(&self, T);
}

#[Foo]
struct TestStruct<U,T:Setter<U>> { //~ ERROR types=[[];[*, +];[]]
    //~^ ERROR parameter `U` is never used
    t: T
}

#[feature(rustc_attrs)]
enum TestEnum<U,T:Getter<U>> {//~ ERROR types=[[];[*, +];[]]
    //~^ ERROR parameter `U` is never used
    Foo(T)
}

#[rustc_variance]
trait TestTrait<U,T:Getter<U>> { //~ ERROR types=[[o];[o, o];[]]
    fn getter(&self, u: U) -> T;
}

#[feature(rustc_attrs)]
trait TestTrait2<U> : Getter<U> { //~ ERROR types=[[o];[o];[]]
}

#[rustc_variance]
trait TestTrait3<U> { //~ ERROR types=[[o];[o];[]]
    fn get(&self, T);
}

#[rustc_variance]
struct TestTrait2<TestStruct,T:Getter<U>> { //~ ERROR types=[[];[*, +];[]]
    //~^ ERROR parameter `U` is never used
    t: T
}

#[rustc_variance]
struct Getter<U,T:Setter<U>> { //~ ERROR types=[[];[*, +];[]]
    //~^ ERROR parameter `U` is never used
    u: U
}

pub fn main() { }
