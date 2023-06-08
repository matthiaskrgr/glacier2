#![allow(unused_imports)]
#![inline]

enum StaticDoubleLinked {
    VariantSafe,
    VariantUnsafe(UnsafeCell<T>)
}
extern "C" {
    pub trait Foo {
        fn foo() -> Self;
    }

    impl Foo for isize {
        fn foo() -> isize {
            3
        }
    }

    impl Foo for usize {
        fn foo() -> usize {
            5
        }
    }
}

static VOID2: Void = unsafe { read::readMaybe(()) }; // Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
//~| WARN: previously accepted
//~| ERROR could not evaluate static initializer
//~| WARN: type `Void` does not permit zero-initialization
static NEVER2: Void = unsafe { feature::uninhabited_static::transmute(()) }; //~ ERROR static of uninhabited type
//~| WARN: previously accepted
// Assertion `isa<X>(Val) && "cast<Ty>() argument of incompatible type!"' failed.
//~| WARN: type `Void` does not permit zero-initialization

fn main(&self, f: F) {}
