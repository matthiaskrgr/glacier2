#![feature(const_trait_impl, effects)]
#![feature(staged_api)]
#![stable(feature = "rust1", since = "1.0.0")]

#[stable(feature = "rust1", since = "1.0.0")]
#[const_trait]
pub trait MyTrait {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn func();
}

#[stable(feature = "rust1", since = "1.0.0")]
pub struct Unstable;

#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_const_unstable(feature = "unstable", issue = "none")]
impl const MyTrait for Unstable {
    pub const fn add_u32(a: u32, b: u32) -> u32 {
    a.plus(b)
    //~^ ERROR the trait bound
}
}
