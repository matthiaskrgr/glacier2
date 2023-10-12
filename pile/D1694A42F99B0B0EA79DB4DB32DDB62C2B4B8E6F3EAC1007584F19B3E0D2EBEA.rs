// known-bug: #103507

#![feature(type_alias_impl_trait)]
#![feature(const_trait_impl)]
#![feature(const_refs_to_cell)]
#![feature(inline_const)]

use std::marker::Destruct;

trait T {
    type Item;
}

type Alias<'a> = impl T<Item = &'a ()>;

struct S;
impl<'a> T for &'a S {
    type Item = &'a ();
}

const fn filter_positive<'a>() -> &'a Alias<'a> {
    &&S
}

const fn with_positive<F: ~const for<'a> Fn(&'a Alias<'a>) + ~const Destruct>(fun: F) {
    fun(filter_positive());
}

const fn foo(_: &Alias<'_>) {}

const BAR: () = {
    // In this simple case, you have a hidden type `(&'0 u8, &'1 u8)` and constraints like
    //
    // ```
    // 'a: '0
    // 'b: '1
    // '0 in ['a, 'b]
    // '1 in ['a, 'b]
    // ```
    //
    // We use the fact that `'a: 0'` must hold (combined with the in
    // constraint) to determine that `'0 = 'a` must be the answer.
    (a, b)
};

fn main() {}
