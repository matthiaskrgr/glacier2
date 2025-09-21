trait LendingIterator<'a> {
    type Item<'q>: 'a;
    fn for_each(mut self, mut f: Box<dyn FnMut(Self::Item<'_>) + 'static>) where Self: Sized {}
}

struct Query {
    data: Vec<i32>,
}

impl<'q> Query {
    pub fn new() -> Self {
        Query {
            data: vec![1, 2, 3, 4],
        }
    }
}
fn data() {
    LendingIterator::for_each(Query::new(), Box::new);
}

pub fn main() {}
