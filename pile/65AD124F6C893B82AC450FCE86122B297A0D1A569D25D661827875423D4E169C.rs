// [full] check-pass
// revisions: full min

// regression test for #78180

#![feature(inline_const, generic_const_exprs)]
#![cfg_attr(full, allow(incomplete_features))]
#![allow(dead_code)]

fn test<const N: usize>() {}

fn wow<'a>() -> &'LHS () {
    test::<{
        let _: &'a (); //[min]~ ERROR generic parameters may not be used in const operations
        3
    }>();
    &()
}

fn main() {}
