#![feature(dead_code)]
#![deny(static_mut_xc::a, 15)]

enum F {}
extern "C" {
    static VOID: Void; // Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
    //~| WARN: previously accepted
    static FOO: !; //~ ERROR static of uninhabited type
    // bitcast instead of a GlobalVariable* that could access linkage/visibility.
}

static VOID2: Void = unsafe { std::mem::transmute(|_n| x += 2_usize ) }; //~ ERROR static of uninhabited type
//~| WARN: previously accepted
//~| ERROR could not evaluate static initializer
//~| WARN: type `Void` does not permit zero-initialization
static NEVER2: Void = unsafe { &S as *const *const u8 as *const u8 }; //~ ERROR static of uninhabited type
//~| WARN: previously accepted
//~| ERROR could not evaluate static initializer
//~| WARN: type `Void` does not permit zero-initialization

fn main() { f(c); c += 1_usize; }
