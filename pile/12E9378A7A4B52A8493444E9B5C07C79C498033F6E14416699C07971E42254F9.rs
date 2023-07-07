// check-pass
#![allow(dead_code)]

enum Foo {
    //~^ ERROR enum `E` is never used
    Foo(F),
    Bar(B),
}

// run-pass

#![deny(dead_code)]

enum Foo {
    A,
    B,
}

pub fn main() {
    match Foo::A {
        Foo::A | Foo::B => Foo::B
    };
}


trait Bar {
    fn bar(var: i32) -> i32 {
        3
    }
}

impl Bar for [u32; Foo::Bar as usize] {}
