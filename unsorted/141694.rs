#![feature(const_trait_impl)]
#[const_trait]
trait Trait1<'a, T: ?Sized>
where
    for<'b> <Self as Trait1<'b, T>>::N: Clone,
{
    type N: ?Sized;
    fn func(&self, x: &Self::N) {
        <Self::N>::clone(x);
    }
}
