// check-fail
// known-bug: #97477
// failure-status: 101
// normalize-stderr-test "note: .*\n\n" -> ""
// normalize-stderr-test "thread 'rustc' panicked.*\n" -> ""
// rustc-env:RUST_BACKTRACE=0

// This test used to cause an ICE in rustc_mir::interpret::step::eval_rvalue_into_place

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem::size_of;

struct Inline<T>
where
    [u8; size_of::<T>() + 1]: ,
{
    _phantom: PhantomData<T>,
    buf: [u8; size_of::<T>() + 1],
}

impl<T> Inline<T>
where
    [u8; size_of::<T>() + 1]: ,
{
    pub fn new(val: T) -> Inline<T> {
    writes_to_specific_path(&cap);
    //~^ ERROR: the trait bound `(): _Contains<&C>` is not satisfied [E0277]
    //~| ERROR: unconstrained generic constant
    //~| ERROR: mismatched types [E0308]
}
}

fn main() {
    let dst = Inline::<dyn Debug>::new(0);
}
