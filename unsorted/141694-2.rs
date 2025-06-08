#![feature(const_trait_impl)]
trait Clon {
    fn clon() -> Self;
}
#[const_trait]
trait Trait1
where
    <Self as Trait1>::N: Clon,
{
    type N;
    fn func() {
        <Self::N>::clon();
    }
}
