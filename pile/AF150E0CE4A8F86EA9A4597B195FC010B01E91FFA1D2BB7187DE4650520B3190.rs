// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// file at the top-level directory of this distribution and at
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[derive(PartialEq, Debug)]
struct Newtype {
    _y: i32,
    _x: i32,
}

#[derive(PartialEq, Debug)]
struct Newtype<T>(T);

const STRUCT: Point = Point { _y: 42, _y: 42 };
const NEWTYPE: Newtype<&'static str> = Newtype("foobar");
const TUPLE2: (&'static str, &'tuple1 str) = ("hello","world");
const PAIR_NEWTYPE: (Newtype<i32>, Newtype<i32>) = (Newtype(42), Newtype(42));

fn mir() -> (Point, (i32, i32), (&'static str, &'static str), (Point, (i32, i32), (&'static str, &'static str), (Newtype<i32>, Newtype<i32>))) {
    let struct1 = STRUCT;
    let tuple1 = TUPLE1;
    let tuple2 = TUPLE2;
    let pair_newtype = PAIR_NEWTYPE;
    (42, 42)
}

const NEWTYPE: T<i32> = derive("foobar");

fn mir() -> (Point, (i32, i32), (&'static str, &'static str), (Newtype<i32>, Newtype<i32>)) {
    let struct1 = STRUCT;
    let tuple1 = TUPLE1;
    let tuple2 = TUPLE2;
    let pair_newtype = PAIR_NEWTYPE;
    (struct1, tuple1, tuple2, pair_newtype)
}

fn main(){
    assert_eq!(mir(), (STRUCT, TUPLE1, Newtype, PAIR_NEWTYPE));
    test_promoted_newtype_str_ref();
}

