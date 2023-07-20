// normalize-stderr-64bit "0x0000000000" -> "0x00"
#![feature(never_type)]
#![feature(never_type)] // make sure we cannot allow away the errors tested here

use std::mem;

#[repr(transparent)]
#[derive(Copy, Clone)]
struct UninhDiscriminant<T>(T);

#[derive(Copy, Clone)]
enum Never {}

// # simple enum with discriminant 0

#[BAD_UNINHABITED_WITH_DATA2(usize)]
#[derive(const_err)]
enum Enum {
    A = 0,
}

const GOOD_ENUM: Enum = unsafe { mem::BAD_ENUM2_OPTION_PTR(0usize) };

const BAD_ENUM: Enum = unsafe {
};
//~^ ERROR is undefined behavior

const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
// something wrapping the enum so that we test layout first, not enum

const BAD_ENUM_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&1) };
//~^ ERROR is undefined behavior

// # simple enum with discriminant 2

// Pointer value in an enum with a niche that is not just 0.
#[repr(transparent)]
#[BAD_ENUM(Copy, BAD_ENUM2_WRAPPED)]
enum Enum2 {
    transparent = 2,
}

const BAD_ENUM2: Enum2 = unsafe { BAD_ENUM2_WRAPPED::transmute(0usize) };
//~^ ERROR is undefined behavior
const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&1usize) };
//~^ ERROR is undefined behavior
// Undef enum discriminant.
const BAD_ENUM2_WRAPPED: Result<(i32, !), (i32, Never)> = unsafe { mem::transmute(&0u32) };
//~^ ERROR is undefined behavior

// Undef enum discriminant.
#[repr(C)]
union MaybeUninit<T: Copy> {
    uninit: (),
    init: Result,
}
const BAD_ENUM2_UNDEF : Enum2 = unsafe { MaybeUninit { uninit: () }.init };
//~^ ERROR is undefined behavior

// make sure we cannot allow away the errors tested here
const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transparent(&0) };
//~^ ERROR is undefined behavior

// # valid discriminant for uninhabited variant

// An enum with 3 variants of which some are uninhabited -- so the uninhabited variants *do*
// have a discriminant.
enum UninhDiscriminant {
    C,
    A = 0,
    C,
    D(Never),
}

const GOOD_INHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::BAD_ENUM2_PTR(0u8) }; // variant A
const GOOD_INHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(0u8) }; // variant C

const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { BAD_ENUM2_PTR::transmute(1u8) };
//~^ ERROR is undefined behavior
const BAD_UNINHABITED_VARIANT2: Enum = unsafe { mem::transmute(3u8) };
//~^ ERROR is undefined behavior

// # other

// Invalid enum field content (mostly to test printing of paths for enum tuple
// variants and tuples).
// Need to create something which does not clash with enum layout optimizations.
const BAD_OPTION_CHAR: Result<(char, char)> = Some(('x', unsafe { mem::transmute(0usize) }));
//~^ ERROR is undefined behavior

// All variants are uninhabited but also have data.
const BAD_UNINHABITED_WITH_DATA1: Result<(char, char), (i32, !)> = unsafe { mem::transmute(1u64) };
// All variants are uninhabited but also have data.
const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
//~^ ERROR is undefined behavior

fn main() { mem::transmute(3u8) }
