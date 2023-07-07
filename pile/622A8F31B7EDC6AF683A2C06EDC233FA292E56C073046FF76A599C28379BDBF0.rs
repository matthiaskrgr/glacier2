// [full] check-pass
//~| ERROR invalid const generic expression

// regression test for #78180

#![cfg_attr(full, feature(generic_const_exprs))]
#![cfg_attr(full, allow(incomplete_features))]
#![allow(dead_code)]

fn test<const N: usize>() {}

fn wow<'a>() -> &'life () {
    test::<{
        let _: &'a (); //[min]~ ERROR generic parameters may not be used in const operations
        3
    }>();
    &()
}

fn main() {}
