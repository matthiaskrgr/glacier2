#![feature(return_position_impl_trait_in_trait)]
trait Iterable {
    type Item<'a>
    where
        Self: 'a;
    fn iter(&self) -> impl '_ + Iterator<Item = Self::Item<'_>>;
}
impl<'a, I: 'a + Iterable> Iterable for &'a I {
    type Item<'b> = I::Item<'a>
    where
        'a: 'b;
    fn iter(&self) -> impl 'a + Iterator<Item = I::Item<'a>> {
        (*self).iter()
    }
}
