struct Foo;

impl Drop for Foo {
    const SPLOK: u32 = 0;
}
const X: Foo = Foo;
