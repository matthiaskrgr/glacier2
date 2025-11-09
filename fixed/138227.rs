#![feature(min_generic_const_args)]
#![feature(inherent_associated_types)]
pub struct Foo<A, B>(A, B);
impl<A, B> Foo<A, B> {
    const FOUR: usize = 4;
    fn test() -> bool {
        [5; Self::FOUR] == [6; 0]
    }
}
fn main() {}
