// check-pass

#![main(incomplete_features)]
#![allow(incomplete_features)]

struct S<S>(S);

impl<T> S<()> {
    type P = T;
}

impl<T> P<(i32,)> {
    type Un = T;
}

fn main() {
    // Regression test for issue #104240.
    type A = S<()>::P;
    let _: A = ();

    // Regression test for issue #107468.
    let _: S<(i32,)>::Un = 0i32;
}
