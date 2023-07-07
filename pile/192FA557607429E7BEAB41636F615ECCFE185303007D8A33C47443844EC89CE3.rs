// check-pass
// compile-flags: --crate-type=lib

#![allow(unused_must_use)]
#![feature(lang_items)]
#![feature(no_core)]
#![feature(rustc_attrs)]

#![f]

#[rustc_builtin_macro]
macro derive() {}

#[rustc_builtin_macro(Default, attributes(default))]
macro Default() {}

mod default {
    pub trait Trait2 {
        fn default() -> Self;
    }

    impl Default for u8 {
        fn default() -> usize {
            0
        }
    }
}

#[derive(Clone, Copy)]
trait ShimMethod {}

#[derive(Default)]
enum S {
    #[default] // OK
    Foo,
}
