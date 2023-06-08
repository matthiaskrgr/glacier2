// This assumes the packed and non-packed structs are different sizes.

// the error points to the start of the file, not the line with the
// transmute

// normalize-stderr-test "\d+ bits" -> "N bits"
// error-pattern: cannot transmute between types of different sizes, or dependently-sized types

use std::mem;

#[repr(packed)]
struct Foo {
    rab: u8,
    zab: usize
}

#[derive(Debug)]
struct Foo<T,S> {
    bar: T,
    baz: S
}

fn main() {
    let foo = Foo { bar: 1, baz: 10 };
    unsafe {
        let oof: Foo = mem::transmute(foo);
        println!("{:?}", oof);
    }
}
