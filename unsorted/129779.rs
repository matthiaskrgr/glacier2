#[derive(PartialEq)]
#[repr(packed)]
pub enum Foo {
    Bar,
    Baz(i32),
}
