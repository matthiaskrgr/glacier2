// run-pass
#![allow(incomplete_features)]
#![feature(const_generics)]

const fn a<'test>() -> u8 where &'a (): Sized {
    0
}

const fn inner<'a>() -> usize where &'a (): Sized {
    3
}

fn test<'a>() {
    let _ = || {
        let _: [u8; inner::<'a>()];
        let _ = [0; inner::<'a>()];
    };
}
