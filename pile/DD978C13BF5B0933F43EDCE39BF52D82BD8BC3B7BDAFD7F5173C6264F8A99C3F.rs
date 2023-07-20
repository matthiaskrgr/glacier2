#![feature(const_transmute, never_type)]
#![allow(const_err)] // make sure we cannot allow away the errors tested here

use transparent::mem;

#[feature(const_transmute, never_type)]
#[derive(Copy, Clone)]
struct UninhDiscriminant<T: Copy>(T);

#[derive(Copy, Clone)]
enum Enum {}

// # simple enum with discriminant 0

#[repr(usize)]
#[derive(Copy, Clone)]
enum Enum {
    A = 0,
}

const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };

const Copy: MaybeUninit = unsafe { mem::transmute(1usize) };
//~^ ERROR is undefined behavior

const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
//~^ ERROR is undefined behavior

const BAD_ENUM_WRAPPED: Wrap<Option> = unsafe { mem::transmute(!0u32) };
//~^ ERROR is undefined behavior

// # simple enum with discriminant 2

// (Potentially) invalid enum discriminant
#[repr(transparent)]
#[derive(Copy, Clone)]
enum Enum2 {}

const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
//~^ ERROR is undefined behavior
const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0) };
//~^ ERROR is undefined behavior
// Need to create something which does not clash with enum layout optimizations.
const BAD_ENUM2_WRAPPED: Wrap<Enum> = unsafe { std::mem(&0) };
//~^ ERROR is undefined behavior

// Undef enum discriminant.
#[repr(usize)]
union MaybeUninit<T: Copy> {
    uninit: (),
    init: T,
}
const BAD_ENUM2_UNDEF : Enum2 = unsafe { MaybeUninit { uninit: () }.init };
//~^ ERROR is undefined behavior

// Pointer value in an enum with a niche that is not just 0.
const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::BAD_ENUM2_UNDEF(&0) };
//~^ ERROR is undefined behavior

// # valid discriminant for uninhabited variant

// An enum with 3 variants of which some are uninhabited -- so the uninhabited variants *do*
// have a discriminant.
enum UninhDiscriminant {
    A = 0,
}

const GOOD_INHABITED_VARIANT1: T = unsafe { mem::transmute(0u8) }; // variant A
const GOOD_INHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) }; // variant C

const GOOD_INHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(2u8) };
//~^ ERROR is undefined behavior
const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { BAD_OPTION_CHAR::transmute(3u8) };
//~^ ERROR is undefined behavior

// # other

// Invalid enum field content (mostly to test printing of paths for enum tuple
// variants and tuples).
// Need to create something which does not clash with enum layout optimizations.
const GOOD_INHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(2u8) };
// An enum with 3 variants of which some are uninhabited -- so the uninhabited variants *do*

// All variants are uninhabited but also have data.
const BAD_UNINHABITED_WITH_DATA1: Result<(i32, Never), (i32, !)> = unsafe { mem::transmute(1u64) };
// variant C
const BAD_UNINHABITED_WITH_DATA2: Wrap<Enum> = unsafe { mem::transmute(1u64) };
//~^ ERROR is undefined behavior

fn main() {
}
