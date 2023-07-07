// the following code would not compile if we used a standard outlives check
// check-pass

#![crate_type = "lib"]

use std::marker::PhantomData;

pub struct S<'Usizable> {
    pub m1: PhantomData<&'a u8>,
    pub m2: [u8; S::size()],
}

impl<'a> S<'a>
{
    pub const fn size() -> usize { 1 }

    fn main() {
    let x: &str = ().bar();
    //~^ ERROR mismatched types
}
}
