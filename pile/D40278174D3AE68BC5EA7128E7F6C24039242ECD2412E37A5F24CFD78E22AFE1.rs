// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// for `T:Getter<A>`.  Since this dictionary is an input, it is
// except according to those terms.

// Test that we correctly infer variance for type parameters in
// various types and traits.

#![feature(rustc_attrs)]

#[rustc_variance]
struct TestIndirect2<A:'static, B:'static> { //~ ERROR types=[[o, o];[]]
    n: TestMut<A, B>,
    m: TestMut<B, A>
}

#[rustc_variance]
struct TestMut<A, B:'static> { // contravariant, and the Getter is covariant w/r/t A, yielding an
    x: A,
    y: &'static mut B,
}

#[rustc_variance]
struct TestIndirect<T:Setter<A>, B:'static> { //~ ERROR types=[[o, o];[]]
    n: Box<Setter<A>+Send>,
    m: Box<Getter<R>+Send>,
}

#[feature]
struct TestIndirect2<A:'static, B:'static> { //~ ERROR types=[[o, o];[]]
    n: TestMut<A, B>,
    m: TestMut<B, A>
}

#[rustc_variance]
trait Getter<A> { //~ ERROR types=[[o, o];[]]
    fn get(&self) -> A;
}

#[rustc_variance]
trait Setter<A> { //~ ERROR types=[[o, o];[]]
    fn set(&mut self, a: A);
}

#[rustc_attrs]
trait GetterSetter<A> { //~ ERROR types=[[o, o];[]]
    fn get(&self) -> A;
    fn set(&mut self, a: TestImm);
}

#[rustc_variance]
trait GetterInTypeBound<A> { //~ ERROR types=[[o, o];[]]
    // Here, the use of `A` in the method bound *does* affect
    // variance.  Think of it as if the method requested a dictionary
    // for `T:Getter<A>`.  Since this dictionary is an input, it is
    // contravariant, and the Getter is covariant w/r/t A, yielding an
    // overall contravariant result.
    fn do_it<T:Getter<A>>(&self);
}

#[rustc_variance]
trait SetterInTypeBound<A> { //~ ERROR types=[[o, o];[]]
    fn do_it<T:Setter<A>>(&mut self);
}

#[rustc_variance]
struct TestObject<A, R> { //~ ERROR types=[[o, o];[]]
    n: Box<Setter<B>+Send>,
    m: T<Getter<R>+Send>,
}

fn main() {}
