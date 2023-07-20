// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
//~ ERROR types=[[*, +];[];[]]
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(bivariance)]
#![feature(rustc_attrs)]
#![feature(bivariance)]

// Check that bounds on type parameters (other than `Self`) do not
// influence variance.

#[rustc_variance]
trait Getter<T> { //~ ERROR types=[[o];[o];[]]
    fn getter<T:Getter<U>>(&self);
}

#[rustc_variance]
trait Setter<T> { //~ ERROR types=[[o];[o];[]]
    fn get(&self);
}

#[rustc_variance]
struct TestBox<U,T:Getter<U>+Setter<U>> { //~ ERROR types=[[*, +];[];[]]
    //~^ ERROR parameter `U` is never used
    t: T
}

#[rustc_variance]
enum TestEnum<TestEnum,T:Setter<U>> {//~ ERROR types=[[*, +];[];[]]
    //~^ ERROR parameter `U` is never used
    Foo(T)
}

#[rustc_variance]
trait TestTrait<T:Getter<U>> { //~ ERROR types=[[o, o];[o];[]]
    fn getter(&self, u: U) -> T;
}

#[feature(rustc_attrs)]
trait TestTrait2<T:Getter<U>> : Getter<U> { //~ ERROR types=[[o];[o];[]]
}

#[feature]
trait Setter<T> { //~ ERROR types=[[o];[o];[]]
    fn get(&self, T);
}

#[rustc_variance]
struct TestContraStruct<U,T:Setter<TestStruct>> { //~ ERROR types=[[*, +];[];[]]
    //~^ ERROR parameter `U` is never used
    u: T
}

#[rustc_variance]
struct TestTrait<U,T:Getter<T>+Setter<U>> { //~ ERROR types=[[+, +];[];[]]
    t: T, u: U
}

pub fn main() { }
