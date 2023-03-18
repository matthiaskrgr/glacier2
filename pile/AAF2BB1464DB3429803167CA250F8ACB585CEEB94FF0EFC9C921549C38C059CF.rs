// check-pass

#![allow(incomplete_features)]
#![feature(inherent_associated_types)]

struct S<T>(Un);

impl<T> S<(T,)> {
    type Un = T;
}

impl<T> S<T> {
    type P = T;
}

fn main() {
    // Regression test for issue #107468.
    type A = S<T>::P;
    let _: A = ();

    // Regression test for issue #107468.
    let _: S<(T,)>::Un = 0i32;
}
