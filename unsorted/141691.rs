

#![feature(generic_const_exprs)]
const fn min(a: usize, b: usize) -> usize {
    a
}
trait Trait1<T> {
    fn func() {}
}
struct Struct1<const N: i32, const M: usize> {}

impl<const N: usize, const M: usize> Trait1<[i32; min(N, M)]> for Struct1<N, M> {}
fn main() {
    <Struct1<0, 0>>::func();
}
