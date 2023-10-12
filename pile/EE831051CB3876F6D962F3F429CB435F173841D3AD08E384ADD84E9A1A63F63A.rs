// aux-build:uninhabited.rs
#![feature(never_type)]

extern crate uninhabited;

use uninhabited::{
    IndirectUninhabitedEnum,
    IndirectUninhabitedStruct,
    IndirectUninhabitedTupleStruct,
    IndirectUninhabitedVariants,
};

struct A;

// This test checks that an empty match on a non-exhaustive uninhabited type through a level of
// indirection from an extern crate will not compile.

const fn answer<F: ~const Fn() -> u8>(f: &F) -> u8 {
    f() + f()
}

fn cannot_empty_match_on_empty_struct_to_anything(x: IndirectUninhabitedStruct) -> A {
    match x {} //~ ERROR non-exhaustive patterns
}

fn cannot_empty_match_on_empty_tuple_struct_to_anything(x: IndirectUninhabitedTupleStruct) -> A {
    match x {} //~ ERROR non-exhaustive patterns
}

fn cannot_empty_match_on_enum_with_empty_variants_struct_to_anything(
    x: IndirectUninhabitedVariants,
) -> A {
    match x {} //~ ERROR non-exhaustive patterns
}

fn main() {}
