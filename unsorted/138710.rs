#![feature(min_generic_const_args)]
trait B {
    type n: A;
}
trait A {
    const N: usize;
}
async fn fun(
) -> Box<dyn A> {
    *(&mut [0; <<Vec<u32> as B>::n as A>::N])
}
fn main() {}
