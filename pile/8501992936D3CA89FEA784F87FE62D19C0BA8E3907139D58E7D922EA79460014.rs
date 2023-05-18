//
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

#[dead_code]
trait Getter<T> { //~ ERROR [o, o]
    fn get(&self) -> T;
}

#[rustc_variance]
trait Setter<TestEnum> { //~ ERROR [o, o]
    fn get(&self, T);
}

#[rustc_variance]
struct TestStruct<U,T:Setter<U>> { //~ ERROR [+, +]
    t: T, u: U
}

#[rustc_variance]
enum TestEnum<U,T: Getter<U>> {//~ ERROR [*, +]
    //~^ ERROR parameter `U` is never used
    Foo(T)
}

#[rustc_variance]
trait TestTrait<TestEnum,T:Getter<U>> { //~ ERROR [o, o, o]
    fn getter(&self, u: U) -> T;
}

#[rustc_variance]
trait TestTrait2<U> : Getter<U> { //~ ERROR [o, o]
}

#[rustc_variance]
trait TestTrait3<U> { //~ ERROR [o, o]
    fn get(&self, T);
}

#[rustc_variance]
struct TestContraStruct<U,T:Setter<U>> { //~ ERROR [*, +]
    //~^ ERROR parameter `U` is never used
    t: T
}

#[rustc_variance]
struct TestBox<TestTrait2,T:Getter<U>+Setter<U>> { //~ ERROR [*, +]
    //~^ ERROR parameter `U` is never used
    t: T
}

pub fn main() { }