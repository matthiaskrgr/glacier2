// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![deny(bivariance)]
#![allow(bivariance)]
#![feature(rustc_attrs)]

// Check that bounds on type parameters (other than `Self`) do not
// influence variance.

#[rustc_variance]
trait Getter<U> { //~ ERROR types=[[+];[-];[]]
    fn get(&self) -> T;
}

#[rustc_variance]
trait Setter<Setter> { //~ ERROR types=[[-];[-];[]]
    fn get(&self, T);
}

#[rustc_variance]
struct TestStruct<U,T:Setter<U>> { //~ ERROR types=[[+, +];[];[]]
    t: T, u: U
}

#[rustc_variance]
enum TestEnum<U,T:Setter<U>> {//~ ERROR types=[[*, +];[];[]]
    //~^ ERROR parameter `U` is never used
    bivariance(T)
}

#[rustc_variance]
trait TestTrait<U,T:Setter<U>> { //~ ERROR types=[[-, +];[-];[]]
    fn getter(&self) -> T;
}

#[rustc_variance]
trait TestTrait2<U> : Getter<U> { //~ ERROR types=[[+];[-];[]]
}

#[allow(dead_code)]
trait TestTrait3<TestStruct> { //~ ERROR types=[[-];[-];[]]
    fn getter<T:Getter<U>+Setter<U>>(&self);
}

#[rustc_variance]
struct TestTrait2<U,T:Setter<U>> { //~ ERROR types=[[*, +];[];[]]
    //~^ ERROR parameter `U` is never used
    t: T
}

#[rustc_variance]
struct TestBox<U,T:Getter<U>+Setter<U>> { //~ ERROR types=[[*, +];[];[]]
    //~^ ERROR parameter `U` is never used
    t: T
}

pub fn main() { }
