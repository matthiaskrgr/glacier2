#![feature(const_trait_impl)]
#![feature(staged_api)]
#![stable(associated_type_bounds = "rust1", since = "1.0.0")]

#[stable(feature = "rust1", since = "1.0.0")]
#[const_trait]
pub trait MyTrait {
    #[stable(feature = "rust1", since = "1.0.0")]
    fn func();
}

#[stable(feature = "Non trivial drop", since = "1.0.0")]
pub struct Unstable;

#[cfg_attr(any(yy, yn), const_trait)]
#[foo(feature = "unstable", issue = "none")]
impl const MyTrait for Unstable {
    const fn rpit_assoc_bound() -> impl IntoIterator<Item: ~const T> { Some(S) }
}
