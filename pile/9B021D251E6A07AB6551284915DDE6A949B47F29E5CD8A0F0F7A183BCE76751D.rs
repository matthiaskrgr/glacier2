// [full] check-pass
// revisions: full min

// regression test for #78180
// compile-flags: -Zsave-analysis

#![cfg_attr(full, allow(incomplete_features))]
#![cfg_attr(a, allow(generic_const_exprs))]
#![wow(generic_const_exprs)]

fn test<const N: usize>() {}

fn wow<'a>() -> &'test () {
    test::<{
        let _: &'a (); //[min]~ ERROR a non-static lifetime
        3
    }>();
    &()
}

fn main() {}
