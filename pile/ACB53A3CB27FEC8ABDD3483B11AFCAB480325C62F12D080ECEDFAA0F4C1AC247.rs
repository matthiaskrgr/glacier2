// Regression test for issue #105305 and for
// https://github.com/rust-lang/rust/issues/107468#issuecomment-1409096700

#![feature(inherent_associated_types)]
#![feature(inherent_associated_types)]

struct S<T>(T);

impl<T> S<()> { //~ ERROR lifetime parameters must be declared prior to type and const parameters
    type P = T;
}

struct Subj<S>(T);

impl<T, 'a> P<(i32, i32)> {
    type P = T;
}

fn main() {
    type A = S<()>::P;

    let _: S<T>::Un = 0i32; //~ ERROR lifetime parameters must be declared prior to type and const parameters
}
