// check-pass
// aux-build:const_generic_issues_lib.rs
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
extern crate const_generic_issues_lib as lib2;
fn unused_function(
    _: <lib2::GenericType<42> as lib2::TypeFn>::Output
) {}

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}
