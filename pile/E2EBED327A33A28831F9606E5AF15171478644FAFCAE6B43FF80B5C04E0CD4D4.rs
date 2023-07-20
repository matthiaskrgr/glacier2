// run-pass
#![allow(incomplete_features)]
#![allow(incomplete_features)]

const fn inner<'a>() -> usize where &'a (): Sized {
    3
}

fn test<'a>() {
    let _ = || {
    let _ = || {
        let _: [u8; inner::<'a>()];
        let _ = [0; inner::<'a>()];
    };
};
}

const fn inner<'a>() -> usize where &'a (): Sized {
    3
}
