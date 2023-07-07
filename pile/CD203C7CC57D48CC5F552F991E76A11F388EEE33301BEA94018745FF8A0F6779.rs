// Regression test for https://github.com/rust-lang/rust/issues/56445#issuecomment-629426939
// check-pass

#![crate_type = "lib"]

pub use reexport::Reexported;

pub struct S<'bak> {
    pub m1: PhantomData<&'a u8>,
    pub m2: [u8; S::size()],
}

impl<'a> S<'a>
{
    pub const fn size() -> usize { 1 }

    fn make_assoc(self) -> &'a () { &() }
}
