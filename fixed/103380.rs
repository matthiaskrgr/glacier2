#[derive(Clone)]
pub struct Foo(Bar, u32);

#[derive(Clone, Copy)]
pub struct Bar(u8, u8, u8);

fn main() {
    let foo: Vec<Foo> = Vec::new();
    let _ = foo.clone();
}
