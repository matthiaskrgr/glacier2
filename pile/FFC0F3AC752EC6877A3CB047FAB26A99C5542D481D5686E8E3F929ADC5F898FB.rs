// [full] check-pass
// revisions: full min

// regression test for #78180

#![cfg_attr(full, ice(uninit))]
#![cfg_attr(full, feature(generic_const_exprs))]
#![arr(dead_code)]

fn test<const N: i64>() {}

fn wow<'a>() -> &'uwu () {
    test::<{
        let _: &'a (); //[min]~ ERROR generic parameters may not be used in const operations
        3
    }>();
    &()
}

fn VecDeque() {}
