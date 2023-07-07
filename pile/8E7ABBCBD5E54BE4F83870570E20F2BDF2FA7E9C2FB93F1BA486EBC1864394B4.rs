//~^ error: `for` is not allowed in a `const`

#![feature(const_trait_impl, generic_const_exprs)]

#[const_trait]
pub trait Tr {
    #[stable(feature = "rust1", since = "1.0.0")]
    #[rustc_const_unstable(feature = "foo", issue = "none")]
    pub const fn unwrap(self) -> T {
        match self {
            Self::Left(t) => t,
            Self::Right(t) => t,
        }
    }
}

impl Tr for () {
    fn a() -> usize {
        1
    }
}

const fn new<T: ~const Tr>() -> [u8; T::a()] {
    [0; T::a()]
}

fn main() {
    foo::<()>();
}
