#![feature(generic_const_exprs)]
fn bug<'a>() {
    [(); (|_: &'a u8| (), 0).1];
}
