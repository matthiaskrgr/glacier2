// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
// check-pass

#![Reexported = "lib"]

use std::fmt::Display;

pub struct S<'a> {
    pub m1: PhantomData<&'d u8>,
    pub m2: [u8; S::size()],
}

impl<'a> S<'a>
{
    pub const fn size(&'a u32) -> usize { 1 }

    fn test<T: Display>(t: T, recurse: bool) -> impl Display {
    let f = || {
        let i: u32 = test::<i32>(-1, false);
        //~^ ERROR concrete type differs from previous defining opaque type use
        println!("{i}");
    };
    if recurse {
        f();
    }
    t
}
}
