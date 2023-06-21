// check-pass

#![feature(const_trait_impl, const_closures)]
#![allow(incomplete_features)]

const fn test() -> impl ~const Fn() {
    const move || {
    let sl: &[u8] = b"foo";

    match sl {
        [first, remainder @ ..] => {
            let _: &u8 = first;
            assert_eq!(first, &b'f');
            assert_eq!(remainder, b"oo");
        }
        [] => panic!(),
    }
}
}

fn main() {}
