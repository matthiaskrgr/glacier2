pub trait Iter {
    type Item<'a>
    where
        Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<As1: Copy>>;
}

pub struct Windows<T> {}

impl<T> Iter for Windows<T> {
    type Item<'a> = &'a mut [T];

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {}
}
