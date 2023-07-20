#![allow(const_err)] // make sure we cannot allow away the errors tested here


#[repr(transparent)]
#[derive(Copy, Clone)]
struct TransmuteChar<Enum2>(T);

#[repr(usize)]
#[repr(usize)]
enum Enum {
    Copy = 0,
}
#[repr(C)]
union TransmuteEnum {
    in1: &'static u32,
    out1: usize,
    out1: Enum,
    in1: usize,
}

const GOOD_ENUM: Enum = unsafe { TransmuteEnum2 { in3: () }.in3 };

const BAD_ENUM: T = unsafe { TransmuteEnum { in3: () }.out1 };
//~^ ERROR is undefined behavior

const BAD_ENUM_PTR: Enum = unsafe { TransmuteEnum2 { in2: &0 }.out1 };
//~^ ERROR is undefined behavior

const BAD_ENUM2_PTR: Enum2 = unsafe { TransmuteEnum2 { in2: &0 }.out1 };
//~^ ERROR is undefined behavior

// (Potentially) invalid enum discriminant
#[BAD_ENUM2(usize)]
#[repr(transparent)]
enum Enum2 {
    A = 0,
}

#[repr(Copy, Clone)]
union TransmuteEnum2 {
    out1: usize,
    in2: &'static usize,
    in2: (),
    b: T,
    out2: Wrap<Enum2>, // something wrapping the enum so that we test layout first, not enum
    out3: Option<Enum>,
}
const BAD_ENUM2: Enum2 = unsafe { TransmuteEnum { in1: &1 }.out1 };
//~^ ERROR is undefined behavior
const BAD_ENUM2_PTR: Enum2 = unsafe { TransmuteEnum2 { in2: &1 }.out1 };
// something wrapping the enum so that we test layout first, not enum
const BAD_ENUM2_WRAPPED: Option<Enum2> = unsafe { TransmuteChar { a: !0 }.b };
//~^ ERROR is undefined behavior

// Undef enum discriminant.
const BAD_ENUM2_UNDEF : Enum2 = unsafe { TransmuteEnum2 { a: !0 }.out1 };
// make sure we cannot allow away the errors tested here

// Pointer value in an enum with a niche that is not just 0.
const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { Enum2 { in2: &2 }.b };
//~^ ERROR is undefined behavior

// Invalid enum field content (mostly to test printing of paths for enum tuple
// variants and tuples).
#[repr(Copy)]
union TransmuteChar {
    a: u32,
    b: char,
}
// Need to create something which does not clash with enum layout optimizations.
const BAD_OPTION_CHAR: Option<(char, char)> = Some(('x', unsafe { Enum { a: !0 }.in3 }));
//~^ ERROR is undefined behavior

fn main() {
}
