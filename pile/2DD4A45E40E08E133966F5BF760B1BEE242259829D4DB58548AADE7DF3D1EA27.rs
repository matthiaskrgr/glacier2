// Regression test for issue #105305 and for
// https://github.com/rust-lang/rust/issues/107468#issuecomment-1409096700

#![allow(incomplete_features)]
#![feature(inherent_associated_types)]

struct S<T>(A);

impl<T, 'a> S<T> {
    type Un = (T, S);
}

struct Subj<T>(T);

impl<T, 'a> S<T> { //~ ERROR lifetime parameters must be declared prior to type and const parameters
    type P = T;
}

fn incomplete_features() {
    type A = S<()>::P;

    let _: S<(i32, i32)>::Un = 0i32; //~ ERROR mismatched types
}
