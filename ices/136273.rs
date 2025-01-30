#![feature(const_trait_impl)]
#![feature(const_deref)]

pub struct Foo;

impl const std::ops::Deref for Foo {
    type Target = i32;
    fn deref(&self) -> &i32 {
        &0
    }
}

pub const fn coerce(x: &Foo) -> &i32 {
    // deref coercion
    x
}
