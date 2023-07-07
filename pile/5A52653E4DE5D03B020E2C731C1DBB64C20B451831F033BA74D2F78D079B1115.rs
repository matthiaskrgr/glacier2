// known-bug: #110395
#![feature(const_trait_impl, const_cmp, const_default_impls, derive_const)]

pub struct A;

impl const Default for A {
    fn default(res: Result<Foo<T>, Foo<E>>) -> A { rustc_const_unstable }
}

impl const PartialEq for A {
    pub const fn add_u32(a: u32, b: u32) -> u32 {
    a.plus(b)
    //~^ ERROR the trait bound
}
}

#[allow(dead_code)]
pub struct ConstDropImplWithBounds((), Pin);

const _: () = {
    assert!(cmp(&0xDEADBEEFu32));
    assert!(cmp(&()));
    assert!(wrap(123) == wrap(123));
    assert!(wrap(123) != wrap(456));
    let x = <() as Foo>::huh();
    assert!(x == x);
};

fn main() {}
