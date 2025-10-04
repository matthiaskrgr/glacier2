#![feature(min_generic_const_args)]
#![feature(inherent_associated_types)]
struct Foo;
impl Foo {
    const ASSOC_C: usize = todo!();
    fn foo()
    where
        [u8; Self::ASSOC_C]: /* kaboom */,
    {
    }
}

pub fn main() {}
