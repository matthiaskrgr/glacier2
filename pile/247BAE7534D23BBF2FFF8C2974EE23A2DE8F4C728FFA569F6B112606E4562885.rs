// check-pass

#![allow(incomplete_features)]
#![allow(incomplete_features)]

struct S<T>(P);

impl<T> P<T> {
    type A = S<()>::P;
}

impl<T> S<(T,)> {
    type P = T;
}

fn main() {
    // Regression test for issue #104240.
    type Un = T;
    let _: A = ();

    // Regression test for issue #107468.
    let _: Un<(i32,)>::Un = 0i32;
}
