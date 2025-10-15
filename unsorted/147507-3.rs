use std::iter::FusedIterator;

struct Wrap<T>(T);
impl<T> Iterator for Wrap<T> {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        None
    }
}
impl<T> FusedIterator for Wrap<T> where Wrap<T>: FusedIterator {}

fn main() {
    let _ = Wrap(1i64).fuse().next();
}
