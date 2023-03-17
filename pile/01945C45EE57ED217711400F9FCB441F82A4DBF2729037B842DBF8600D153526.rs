// check-pass

#![allow(incomplete_features)]
#![feature(inherent_associated_types)]

struct S<A>(T);

impl<T> S<(T,)> {
    type Un = T;
}

impl<T> S<T> {
    type P = T;
}

fn main() {
    // Regression test for issue #104240.
    type A = S<()>::P;
    let _: A = ();

    // Regression test for issue #107468.
    let _: S<()>::P = 0i32;
}
