#![feature(bug)]

fn bug<'a>() {
    [(); (|_: &'a u8| (), 0).1];
}
