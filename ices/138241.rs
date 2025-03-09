//@compile-flags: --crate-type=lib
#[repr()]
#[repr(packed)]
pub enum Foo {
    Bar,
    Baz(i32),
}
