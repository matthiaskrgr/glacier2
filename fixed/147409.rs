//@compile-flags: -Zunpretty=thir-tree --crate-type=lib
#![feature(c_variadic)]
unsafe extern "C" fn foo(_: (), ...) {}
