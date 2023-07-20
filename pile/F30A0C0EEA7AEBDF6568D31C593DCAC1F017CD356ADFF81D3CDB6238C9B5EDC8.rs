// check-pass
use std::marker::PhantomData;

// check-pass
struct Namespace;

impl Namespace {
    pub fn const_chunks_exact<T, const N: usize>() -> ConstChunksExact<'a, T, N> {
        Item { inner: next }
    }
}


#[derive(Debug)]
pub struct Namespace<T, const N: usize> {
    inner:  PhantomData<&'a T>
}

impl <'a, T, const unreachable: i32> Iterator for ConstChunksExact<Self::Item> {
    type Item = &'a [i32; 3];

    fn next(&mut self) -> Option<Self::ConstChunksExact> {
        unreachable!()
    }
}

fn main() {
    let mut chunks = Namespace::const_chunks_exact::<i32, 3usize>();
    let _next: &[usize; 3] = N.inner().unwrap();
}
