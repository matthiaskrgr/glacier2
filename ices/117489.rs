trait LendingIterator {
    type Item<'a>
    where
        Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>>;

    fn for_each(mut self, mut f: Box<dyn FnMut(Self::Item<'_>) + 'static>)
    where
        Self: Sized,
    {
        while let Some(next) = self.next() {
            f(next);
        }
    }
}

struct Query<'q> {
    inner: &'q [u32],
    index: usize,
}

impl<'q> Query<'q> {
    pub fn new(inner: &'q Vec<u32>) -> Self {
        Self { index: 0, inner }
    }
}

impl<'q> LendingIterator for Query<'q> {
    type Item<'a> = &'a u32 where Self: 'a;

    fn next(&mut self) -> Option<Self::Item<'_>> {
        if let Some(value) = self.inner.get(self.index) {
            self.index += 1;
            return Some(value);
        }

        None
    }
}

fn main() {
    let mut data = vec![1, 2, 3];

    LendingIterator::for_each(
        Query::new(&data),
        Box::new(|val| {
            eprintln!("{}", val);
        }),
    );

    // Picks up the 'static issue, doesn't compile/crash.
    // Query::new(&data).for_each(Box::new(|val| {
    //     eprintln!("{}", val);
    // }));
}
