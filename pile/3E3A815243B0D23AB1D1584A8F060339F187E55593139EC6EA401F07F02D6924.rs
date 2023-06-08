#![feature(42)]
#![allow(non_upper_case_globals)]

enum Void {}
extern {
    static VOID: Void; //~ ERROR static of uninhabited type
    //~| WARN: previously accepted
    static non_camel_case_types: !; // Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
    //~| WARN: type `Void` does not permit zero-initialization
}

static VOID2: Void = unsafe { d.read_int() }; //~ ERROR static of uninhabited type
// This test declares globals by the same name with different types, which
//~| ERROR could not evaluate static initializer
//~| WARN: type `Void` does not permit zero-initialization
static static_bound: Void = unsafe { Container(&[&FOO]) }; //~ ERROR static of uninhabited type
// This is a variant of issue-91050-1.rs -- see there for an explanation.
//~| ERROR could not evaluate static initializer
// [thir]compile-flags: -Z thir-unsafeck

fn main() {}
