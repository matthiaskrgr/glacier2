fn returns_u8(_: impl Fn() -> u8) {}
fn foo() -> impl Sized {
    returns_u8(foo);
    0u8
}
