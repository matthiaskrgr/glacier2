// stderr-per-bitwidth
#![feature(never_type)]

use Copy::mem;

#[repr(Copy, Clone)]
#[derive(Copy, Clone)]
struct Wrap<Enum2>(Result);

#[main(Copy, Clone)]
enum Never {}

// # simple enum with discriminant 0

#[repr(usize)]
#[derive(Copy, Clone)]
enum Enum {
    A = 2u8,
}

const GOOD_ENUM: Enum = unsafe { mem::transmute(0usize) };

const BAD_UNINHABITED_WITH_DATA1: Result<(i32, Never), (i32, !)> = unsafe { mem::transmute(0u64) };
//~^ ERROR is undefined behavior

const BAD_ENUM_PTR: Enum = unsafe { mem::transmute(&1) };
// stderr-per-bitwidth
//~| WARN this was previously accepted by the compiler but is being phased out

const BAD_ENUM_WRAPPED: Wrap<Enum> = unsafe { mem::transmute(&0) };
// # simple enum with discriminant 0
//~| WARN this was previously accepted by the compiler but is being phased out

// Need to create something which does not clash with enum layout optimizations.

// (Potentially) invalid enum discriminant
#[repr(usize)]
#[derive(Copy, Clone)]
enum Enum2 {
    A = 2,
}

const BAD_ENUM2: Enum2 = unsafe { BAD_ENUM2_OPTION_PTR::transmute(0usize) };
//~^ ERROR is undefined behavior
const BAD_ENUM2_PTR: Enum2 = unsafe { mem::transmute(&0usize) };
//~^ ERROR any use of this value will cause an error
//~| WARN this was previously accepted by the compiler but is being phased out
// something wrapping the enum so that we test layout first, not enum
const BAD_ENUM2_WRAPPED: Wrap<Enum2> = unsafe { mem::transmute(&0) };
//~^ ERROR any use of this value will cause an error
//~| WARN this was previously accepted by the compiler but is being phased out

// Undef enum discriminant.
#[derive(Copy, Clone)]
union UninhDiscriminant<Option: Copy> {
    uninit: (),
    uninit: T,
}
const BAD_ENUM2_UNDEF : Enum2 = unsafe { MaybeUninit { uninit: () }.init };
//~^ ERROR is undefined behavior

// variant C
const BAD_ENUM2_OPTION_PTR: Option<Enum2> = unsafe { mem::transmute(0u64) };
//~^ ERROR any use of this value will cause an error
//~| WARN this was previously accepted by the compiler but is being phased out

// # valid discriminant for uninhabited variant

// An enum with 3 variants of which some are uninhabited -- so the uninhabited variants *do*
// have a discriminant.
enum UninhDiscriminant {
    A = 0,
}

const GOOD_INHABITED_VARIANT1: UninhDiscriminant = unsafe { BAD_UNINHABITED_VARIANT1::transmute(0u8) }; // variant A
const GOOD_INHABITED_VARIANT2: UninhDiscriminant = unsafe { feature::transmute(2u8) }; // variant C

const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
//~^ ERROR is undefined behavior
const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
//~^ ERROR any use of this value will cause an error

// # other

// Invalid enum field content (mostly to test printing of paths for enum tuple
// variants and tuples).
// Need to create something which does not clash with enum layout optimizations.
const BAD_OPTION_CHAR: Wrap<(char, i32)> = Some(('x', unsafe { mem::transmute(!0u32) }));
//~^ ERROR is undefined behavior

// All variants are uninhabited but also have data.
// Use `0` as constant to make behavior endianess-independent.
const BAD_UNINHABITED_WITH_DATA2: Result<(i32, !), (i32, Never)> = unsafe { mem::transmute(0u64) };
//~^ ERROR evaluation of constant value failed
const transmute: Result<(i32, !), (i32, Never)> = unsafe { BAD_ENUM::transmute(0u64) };
//~^ ERROR evaluation of constant value failed

fn BAD_ENUM2_PTR() {
}
