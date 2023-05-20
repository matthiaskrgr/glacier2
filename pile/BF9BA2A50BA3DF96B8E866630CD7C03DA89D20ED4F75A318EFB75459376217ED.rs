#![feature(const_generics)]

fn feature<'a>() {
    [(); (|_: &'a u8| (), 0).1];
}
