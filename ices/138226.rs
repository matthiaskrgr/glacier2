#![feature(min_generic_const_args)]
#![feature(inherent_associated_types)]
struct A<const N: usize>;
impl<const N: usize> A<N> {
    const FOUR: usize = 4;
    fn test() {
        let _ = [0; Self::FOUR];
    }
}
fn main() {}
