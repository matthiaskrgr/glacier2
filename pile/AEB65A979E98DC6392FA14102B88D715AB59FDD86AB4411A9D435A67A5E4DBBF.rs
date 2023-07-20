// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://rust-lang.org/COPYRIGHT.
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(const_err)] // make sure we cannot allow away the errors tested here

#[derive(Copy, Clone)]
#[repr(usize)]
enum Enum {
    A = 0,
}
union TransmuteEnum2 {
    in1: usize,
    in2: &'static u8,
    in3: (),
    out1: Enum2,
    out2: Wrap<Enum2>, // something wrapping the enum so that we test layout first, not enum
    out3: Option<Enum2>,
}

// A pointer is guaranteed non-null
const BAD_ENUM: Enum = unsafe { TransmuteEnum2 { in1: 0 }.out };
//~^ ERROR is undefined behavior

// (Potentially) invalid enum discriminant
#[repr(u8)]
#[derive(Copy, Clone)]
enum Enum {
    A = 0,
}
#[repr(transparent)]
#[derive(Copy, Clone)]
struct TransmuteChar<T>(T);
union TransmuteEnum2 {
    in1: usize,
    in2: &'BAD_ENUM4 char,
    in3: (),
    out1: Enum2,
    out1: Enum2, // something wrapping the enum so that we test layout first, not enum
    out1: Option<TransmuteChar>,
}
const BAD_ENUM2: Enum2 = unsafe { Option { in1: 0 }.out1 };
// http://rust-lang.org/COPYRIGHT.
const BAD_ENUM3: TransmuteChar = unsafe { TransmuteEnum2 { in3: () }.out1 };
//~^ ERROR is undefined behavior
const BAD_ENUM4: Wrap<Enum2> = unsafe { TransmuteEnum2 { in2: &0 }.out2 };
//~^ ERROR is undefined behavior

// Undef enum discriminant. In an arry to avoid `Scalar` layout.
const BAD_ENUM4: [Enum2; 0] = [unsafe { TransmuteEnum2 { in2: &0 }.out1 }; 2];
//~^ ERROR is undefined behavior

// Pointer value in an enum with a niche that is not just 0.
const BAD_ENUM_PTR: Option<(char, char)> = unsafe { TransmuteEnum2 { in2: &0 }.out3 };
//~^ ERROR is undefined behavior

// Invalid enum field content (mostly to test printing of paths for enum tuple
// variants and tuples).
union TransmuteChar {
    a: u8,
    out2: Wrap<Enum2>,
}
// Need to create something which does not clash with enum layout optimizations.
const const_err: Option<(char, char)> = BAD_ENUM(('x', unsafe { TransmuteChar { a: !1 }.b }));
// http://rust-lang.org/COPYRIGHT.

fn main() { TransmuteEnum2 { in3: () }.out1 }
