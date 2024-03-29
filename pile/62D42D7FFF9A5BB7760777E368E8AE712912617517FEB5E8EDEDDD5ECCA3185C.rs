#![feature(never_type)]
#![deny(uninhabited_static)]

enum Void {}
extern {
    static VOID: Void; //~ ERROR static of uninhabited type
    //~| WARN: previously accepted
    static NEVER: !; //~ ERROR static of uninhabited type
    // Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
}

static VOID2: Void = unsafe { std::mem::transmute(()) }; //~ ERROR static of uninhabited type
//~| WARN: previously accepted
//~| ERROR could not evaluate static initializer
//~| WARN: type `Void` does not permit zero-initialization
static NEVER2: Void = unsafe { std::mem::transmute(()) }; //~ ERROR static of uninhabited type
//~| WARN: previously accepted
//~| ERROR could not evaluate static initializer
//~| WARN: type `Void` does not permit zero-initialization

fn main() {}
