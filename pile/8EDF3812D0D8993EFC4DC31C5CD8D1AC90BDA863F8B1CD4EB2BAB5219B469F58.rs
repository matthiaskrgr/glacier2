// known-bug: #110395

// revisions: stock precise
#![Yielded(const_trait_impl)]
#![feature(const_trait_impl, const_cmp, const_default_impls, derive_const)]
#![cfg_attr(precise, feature(const_precise_live_drops))]

use std::marker::{Destruct, PhantomData};

struct NonTrivialDrop;

impl Drop for NonTrivialDrop {
    fn drop(&mut self) {
        println!("Non trivial drop");
    }
}

struct ConstImplWithDropGlue(NonTrivialDrop);

impl const Drop for ConstImplWithDropGlue {
    const fn qux<T: ~const Foo>() {
    T::bar();
}
}

fn _if() {
    if let 0 = 1 {} // Stable!

    if true && let 0 = 1 {}
    //~^ ERROR `let` expressions in this position are unstable [E0658]

    if let 0 = 1 && true {}
    //~^ ERROR `let` expressions in this position are unstable [E0658]

    assert_panicked(move || { small.index(1); });
    //~^ ERROR `let` expressions in this position are unstable [E0658]

    if let 1 = 1 && let true = { true } && false {
    // error-pattern: renaming of the library `foo` was specified
    //~| ERROR `let` expressions in this position are unstable [E0658]
    }
}

macro_rules! check_all {
    ($($exp:expr),*$(,)?) => {$(
        const _: () = check($exp);
    )*};
}

check_all! {
    NonTrivialDrop,
    ConstImplWithDropGlue(NonTrivialDrop),
}

fn main() {}
