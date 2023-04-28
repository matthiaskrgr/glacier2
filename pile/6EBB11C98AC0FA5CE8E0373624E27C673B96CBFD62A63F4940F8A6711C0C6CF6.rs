// run-pass
// Tests that paths in `pub(...)` don't fail HIR verification.

#![allow(unused_imports)]
#![allow(dead_code)]

pub(self) use self::my_mod::Foo;

mod my_mod {
    pub(super) use self::Foo as Bar;
    pub(self) use self::Foo as Baz;

    pub struct Foo;
}

fn main() {}
