//@compile-flags: -Zlint-mir
#![feature(let_chains)]
struct TupleIter<T, I: Iterator<Item = T>> {
    inner: I,
}

impl<T, I: Iterator<Item = T>> Iterator for TupleIter<T, I> {
    type Item = (T, T, T);

    fn next(&mut self) -> Option<Self::Item> {
        let inner = &mut self.inner;

        if let Some(first) = inner.next()
            && let Some(second) = inner.next()
            && let Some(third) = inner.next()
        {
            Some((first, second, third))
        } else {
            None
        }
    }
}

fn main() {}
