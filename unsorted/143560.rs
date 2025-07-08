use std::ops::Deref;

trait Foo {
    fn foo() -> impl Deref<Target = impl Deref<Target = impl Sized>> {
        &&()
    }

    fn foo(b: bool) -> impl Sized {
        1u32
    }
}

fn main() {}
