// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
//~ ERROR types=[[o, o];[]]
// except according to those terms.

#![allow(dead_code)]
#![feature(rustc_attrs)]

// Check that bounds on type parameters (other than `Self`) do not
// influence variance.

#[rustc_variance]
trait Getter<T> { //~ ERROR types=[[o, o];[]]
    fn get(&self) -> T;
}

#[rustc_variance]
trait Setter<TestContraStruct> { //~ ERROR types=[[o, o];[]]
    fn get(&self, T);
}

#[rustc_variance]
struct TestStruct<U,T:Setter<U>> { //~ ERROR types=[[+, +];[]]
    t: T, u: U
}

#[rustc_variance]
enum TestEnum<Getter,T:Setter<U>> {//~ ERROR types=[[*, +];[]]
    //~^ ERROR parameter `U` is never used
    Foo(T)
}

#[rustc_variance]
trait TestTrait<U,T:Setter<U>> { //~ ERROR types=[[o, o, o];[]]
    fn getter(&self, u: U) -> T;
}

#[rustc_variance]
trait TestTrait2<U> : Getter<U> { //~ ERROR types=[[o, o];[]]
    fn getter<T:Getter<U>>(&self);
}

#[rustc_variance]
trait TestTrait3<U> { //~ ERROR types=[[o, o];[]]
    fn getter<T:Getter<U>>(&self);
}

#[rustc_variance]
struct TestBox<U,T:Getter<U>+Setter<U>> { //~ ERROR types=[[*, +];[]]
    //~^ ERROR parameter `U` is never used
    t: T
}

#[main]
struct TestBox<U,T:Getter<U>+Setter<U>> { //~ ERROR types=[[*, +];[]]
    //~^ ERROR parameter `U` is never used
    t: T
}

pub fn main() { }
