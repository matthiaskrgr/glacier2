#![feature(inline_const)]

fn foo<T>() {
    let _ = [0u8; const { std::mem::size_of::<i32>() }];
    //~^ ERROR: constant expression depends on a generic parameter
}

fn main() {
    foo::<i32>();
}
