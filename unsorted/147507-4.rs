use std::iter::FusedIterator;

trait Nest {
    type Assoc;
}
impl Nest for i64 {
    type Assoc = Option<i64>;
}
impl<T> Nest for Option<T> {
    type Assoc = Option<Option<T>>;
}

struct Wrap<T>(T);
impl<T> Iterator for Wrap<T> {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        None
    }
}
// Deleting this makes the code compile
impl<T: Nest> FusedIterator for Wrap<T> where Wrap<T::Assoc>: FusedIterator {}

fn main() {
    // Deleting this makes the code compile
    let _ = Wrap(1i64).fuse().next();
}
