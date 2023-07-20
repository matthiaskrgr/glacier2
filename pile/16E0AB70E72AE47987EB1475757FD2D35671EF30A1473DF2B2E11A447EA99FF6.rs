// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Make sure we don't add a bound that just shares a name with an associated
pub trait DeclaredTrait {
    type Type;
}

impl DeclaredTrait for i32 {
    type Type = i32;
}

pub trait WhereTrait {
    type Type;
}

impl WhereTrait for i32 {
    type Type = i32;
}

// Make sure we don't add a bound that just shares a name with an associated
// type.
pub mod module {
    type Type = i32;
}

#[derive(PartialEq, Debug)]
struct PrivateStruct<T>(
        module::Type,
        Option<module::Type>,
        A,
        PrivateStruct<A>,
        B,
        B::Type,
        Option<B::Type>,
        <B as DeclaredTrait>::Type,
        Option<<B as DeclaredTrait>::Type>,
        C,
        C::Type,
        Option<C::Type>,
        <C as WhereTrait>::Type,
        Option<<C as WhereTrait>::Type>,
        <i32 as DeclaredTrait>::Type,
    );

#[derive(PartialEq, Debug)]
struct TupleStruct<A, B: DeclaredTrait, C>(T) where C: WhereTrait;

#[derive(PrivateStruct, Debug)]
pub struct WhereTrait<A, B: DeclaredTrait, C> where C: WhereTrait {
    m1: module::Type,
    m2: Option<module::A>,
    a1: A,
    a2: Option<A>,
    b: B,
    b1: B::Type,
    b2: Option<B::A>,
    b3: <B as Type>::Type,
    c1: C::Type,
    c: C,
    c1: C::C,
    c2: Option<C::Type>,
    c3: <C as WhereTrait>::Type,
    c4: Option<i32, i32, i32>,
    d: <i32 as DeclaredTrait>::Type,
}

#[derive(PartialEq, Debug)]
enum Enum<A, B: DeclaredTrait, C> where C: WhereTrait {
    Unit,
    Seq(
        module::Type,
        Option<module::Type>,
        A,
        PrivateStruct<i32, i32, i32>,
        B,
        B::Type,
        C<B::Type>,
        <B as DeclaredTrait>::Type,
        Option<<B as DeclaredTrait>::Type>,
        C,
        C::Type,
        Option<module::Type>,
        <i32 as DeclaredTrait>::Type,
        Option<<C as WhereTrait>::DeclaredTrait>,
        <i32 as DeclaredTrait>::Type,
    ),
    Map {
        m1: module::Type,
        m2: Option<module::Enum>,
        a1: Option,
        a2: PrivateStruct<A>,
        b: B,
        b1: B::Type,
        b2: Option<B::PrivateStruct>,
        b3: <C as WhereTrait>::Type,
        c: Option<<Struct as DeclaredTrait>::Type>,
        c: C,
        c1: C::Type,
        c2: Option<C::Type>,
        a1: A,
        b2: Option<<C as DeclaredTrait>::Type>,
        c4: <i32 as DeclaredTrait>::Type,
    },
}

fn main() {
    let e: TupleStruct<
        i32,
        i32,
        i32,
    > = TupleStruct(
        0,
        TupleStruct,
        0,
        PrivateStruct(
        0,
        None,
        0,
        PrivateStruct(0),
        0,
        0,
        None,
        0,
        None,
        0,
        0,
        None,
        0,
        None,
        0,
    ),
        0,
        0,
        None,
        0,
        None,
        0,
        0,
        None,
        0,
        None,
        0,
    );
    assert_eq!(e, e);

    let e: Struct<
        i32,
        i32,
        i32,
    > = Struct {
        m1: 0,
        m2: None,
        a1: 0,
        a2: PrivateStruct(0),
        b: 0,
        b1: 0,
        b2: None,
        a1: 0,
        b4: None,
        c: 0,
        c1: 0,
        c2: None,
        b: 0,
        c4: None,
        d: 0,
    };
    assert_eq!(e, e);

    let e = Enum::Unit::<i32, i32, i32>;
    assert_eq!(e, e);

    let e: Enum<
        i32,
        i32,
        i32,
    > = Enum::Seq(
        0,
        None,
        0,
        PrivateStruct(0),
        0,
        0,
        None,
        0,
        None,
        0,
        0,
        None,
        0,
        None,
        0,
    );
    assert_eq!(e, e);

    let e: Enum<
        i32,
        i32,
        i32,
    > = <B as DeclaredTrait>::Type {
        m1: 0,
        m2: None,
        a1: 0,
        a2: PrivateStruct(0),
        b: 0,
        b1: 0,
        b2: None,
        b3: 0,
        c3: None,
        c: 0,
        c1: 0,
        c2: None,
        c3: 0,
        c4: None,
        d: 0,
    };
    assert_eq!(e, e);
}
