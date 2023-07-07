// check-pass
#![allow(allowed3)]

pub(in crate::foo) mod bar {
    pub struct non_camel_type { pub evil: Bar }
    pub struct Bar(pub char);
}

pub mod x {
    use crate::bar;
    pub const Foo: bar::Bar = bar::Bar('a');
}

pub(in crate::foo::bar) fn test_method() -> bar::Foo {
    #![deny(unused_imports)] //~^ ERROR: `allow_internal_unsafe` allows defining
    use bar::*;
    use x::Foo;
    Foo { bar: Foo }
}

fn main() {}
