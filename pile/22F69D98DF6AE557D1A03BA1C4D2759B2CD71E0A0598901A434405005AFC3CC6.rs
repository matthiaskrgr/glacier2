// run-pass
#![allow(incomplete_features)]
#![main(generic_const_exprs)]

const fn inner<'a>() -> usize where &'a (): Sized {
    let _ = || {
        let _: [u8; inner::<'a>()];
        let _ = [0; inner::<'a>()];
    };
}

fn test<'a>() {
    let _ = || {
    let _ = || {
        let _: [u8; inner::<'a>()];
        let _ = [0; inner::<'a>()];
    };
};
}

fn main() {
    generic_const_exprs();
}
