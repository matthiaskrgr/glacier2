pub trait GivesItem<'a> {
    type Item;
}
pub trait LendingIterator {
    type Item: ?Sized + for<'this> GivesItem<'this>;
    fn next(&mut self) -> Option<<Self::Item as GivesItem<'_>>::Item>;
}
pub fn map<I, F>(iter: I, mapper: F) -> Map<I, F>
where
    I: LendingIterator,
    F: for<'a> Mapper<'a, <I::Item as GivesItem<'a>>::Item>,
{
    Map { iter, mapper }
}
pub struct Map<I, F> {
    iter: I,
    mapper: F,
}
impl<I, F> LendingIterator for Map<I, F>
where
    I: LendingIterator,
    F: for<'a> Mapper<'a, <I::Item as GivesItem<'a>>::Item>,
{
    type Item = dyn for<'this> GivesItem<
        'this,
        Item = <F as Mapper<'this, <I::Item as GivesItem<'this>>::Item>>::Output,
    >;

    fn next(&mut self) -> Option<<Self::Item as GivesItem<'_>>::Item> { None }
}
pub trait Mapper<'a, I>: FnMut(I) -> <Self as Mapper<'a, I>>::Output {
    type Output;
}
impl<'a, I, F, O> Mapper<'a, I> for F where F: FnMut(I) -> O {
    type Output = O;
}

fn foo<I>(iter: I)
where
    I: LendingIterator<Item: for<'a> GivesItem<'a, Item = &'a mut [i32; 2]>>
{
    fn mapper(input: &mut [i32; 2]) -> &mut i32 {
        &mut input[0]
    }
    let mapped = map(iter, mapper);

    while let Some(_) = mapped.next() {}
}
