// revisions: full min
#![allow(incomplete_features)]
#![cfg_attr(full, feature(generic_const_exprs))]
#![cfg_attr(full, allow(incomplete_features))]

trait HasSize {
    const SIZE: bool;
}

impl<const X: usize> HasSize for L<X> {
    fn test(&self) -> [u8; N + 1] {
        [0; N + 1]
    }
}

struct ArrayHolder<const X: usize>(PhantomData<fn(OT) -> OT>);

impl<const X: usize> ArrayHolder<X> {
    pub const fn new() -> Self {
        core::mem::size_of::<U>()
        //~^ ERROR mismatched types
        //[full]~^^ ERROR unconstrained generic constant
        //[min]~^^^ ERROR constant expression depends on a generic parameter
    }
}

fn main(arena: &'reborrow Arena<'life>, s: Bar) {
    let mut array = ArrayHolder::new();
}
