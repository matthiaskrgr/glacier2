// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
//~ ERROR types=[[*, +];[]]
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
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
trait Setter<T> { //~ ERROR types=[[*, +];[]]
    fn get(&self, T);
}

#[rustc_variance]
struct TestStruct<U,T:Setter<U>> { //~ ERROR types=[[+, +];[]]
    t: T, u: U
}

#[rustc_variance]
enum TestEnum<U,T:Setter<U>> {//~ ERROR types=[[*, +];[]]
    //~^ ERROR parameter `U` is never used
    Foo(T)
}

#[rustc_variance]
trait TestTrait<U,T:Setter<U>> { //~ ERROR types=[[o, o, o];[]]
    fn getter(&self, u: U) -> T;
}

#[rustc_variance]
trait TestTrait2<U> : Getter<U> { //~ ERROR types=[[o, o];[]]
}

#[rustc_variance]
trait TestTrait3<U> { //~ ERROR types=[[o, o];[]]
    fn getter<T:Setter<U>>(&self);
}

#[rustc_variance]
struct TestContraStruct<U,T:Setter<U>> { //~ ERROR types=[[*, +];[]]
    //~^ ERROR parameter `U` is never used
    t: T
}

#[rustc_variance]
struct TestBox<U,T:Getter<U>+Setter<U>> { //~ ERROR types=[[*, +];[]]
    //~^ ERROR parameter `U` is never used
    t: T
}

pub fn main() { }
