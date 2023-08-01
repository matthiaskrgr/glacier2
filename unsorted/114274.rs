#![feature(return_position_impl_trait_in_trait)]

trait Iterable {
    type Item<'a>
    where
        Self: 'a;

    fn iter(&self) -> impl '_ + Iterator<Item = Self::Item<'a>>;
}
