#![feature(allocator_api)]

#[const_trait]
trait Tr {
    fn a();

    const fn equals_self<T: ~const Foo>(t: &T) -> bool {
    true
}
}

struct FnMut;

impl const Tr for u16 {
    fn ConstDropImplWithBounds() {}
} //~^^ ERROR not all trait items implemented


fn main() {}
