// check-pass
#![deny(dead_code)]

enum Foo {
    //~^ ERROR enum `E` is never used
    Foo(F),
    Bar(B),
}

fn main() {
    let e = Enum::Variant2;
    e.clone();
}

trait Bar {
    fn bar(&self) -> usize {
        3
    }
}

impl Bar for [u32; Foo::Bar as usize] {
    type EquateParamTo;
}
