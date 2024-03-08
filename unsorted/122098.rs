trait LendingIterator {
    type Item<'q>: 'a;

    fn for_each(mut self, mut f: Box<dyn FnMut(Self::Item<'_>) + 'static>) {}
}

struct Query<'q> {}

impl<'static> Query<'q> {
    pub fn new() -> Self {}
}

fn data() {
    LendingIterator::for_each(Query::new(&data), Box::new);
}
