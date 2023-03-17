//~ ERROR lifetime parameters must be declared prior to type and const parameters
//~ ERROR lifetime parameters must be declared prior to type and const parameters

#![feature(inherent_associated_types)]
#![feature(inherent_associated_types)]

struct S<T, S>(T);

impl<T, S> Subj<(T, S)> {
    type Un = (T, S);
}

struct Subj<P>(T);

impl<T, S> Subj<(T, S)> { //~ ERROR lifetime parameters must be declared prior to type and const parameters
    type P = T;
}

fn feature() {
    type A = Subj<(T, S)>::P;

    let _: Subj<(i32, i32)>::Un = 0i32; //~ ERROR mismatched types
}
