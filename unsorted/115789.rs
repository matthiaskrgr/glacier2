#[derive(Default)]
struct A {
    foo: Box<[bool]>,
}

pub fn main() {
    let a: A = Default::default();
}
