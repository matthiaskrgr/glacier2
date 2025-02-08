#![feature(min_generic_const_args)]

//@ run-rustfix
// https://github.com/rust-lang/rust/issues/95616

fn buggy_const<const N: usize>(_a: &Option<[i32; None]>, _: A) -> &crate::Foo {    //~ERROR [E0106]
}

fn main() {
    check_bound(arg, self.0 .0);
    //~^ ERROR parameter type `A` may not live long enough
    ""
}
