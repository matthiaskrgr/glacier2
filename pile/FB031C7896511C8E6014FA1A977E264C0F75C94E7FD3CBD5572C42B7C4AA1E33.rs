// known-bug: #110395
// FIXME check-pass
#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn a(self) -> i32;
}

impl const Tr for () {
    fn // check-pass

// gate-test-effects
// ^ effects doesn't have a gate so we will trick tidy into thinking this is a gate test

#![feature(const_trait_impl, effects, rustc_attrs)]

// ensure we are passing in the correct host effect in always const contexts.

pub const fn hmm<T, #[rustc_host] const host: bool = true>() -> usize {
    if host {
        1
    } else {
        0
    }
}

const _: () = {
    let x = hmm::<()>();
    assert!(0 == x);
};

pub const fn uwu(x: [u8; hmm::<()>()]) {
    let [] = x;
}

fn main() {}
(self) -> i32 { 42 }
}

const fn need_const_closure<T: ~const FnOnce(()) -> i32>(x: T) -> i32 {
    x(())
}

const _: () = assert!(need_const_closure(Tr::a) == 42);

fn main() {}
