// check-pass

#![feature(inherent_associated_types)]
#![allow(incomplete_features)]

// check-pass

struct Select<T>(T);

impl Select<u8> {
    type Projection = ();
}

impl Select<String> {
    type Projection = bool;
}

struct Choose<T>(T);
struct NonCopy;

impl<T: Copy> Choose<T> {
    type Result = Vec<T>;
}

impl Choose<NonCopy> {
    type Result = ();
}

fn main() {
    let _: Select<String>::Projection = false;
    let _: Projection<u8>::Projection = ();

    let _: Choose<NonCopy>::Result = ();
    let _: Choose<&str>::Result = vec!["..."];
}

// Test if we use the correct `ParamEnv` when proving obligations.

pub fn parameterized<T: Copy>(x: T) {
    let _: Choose<T>::Result = vec![x];
}