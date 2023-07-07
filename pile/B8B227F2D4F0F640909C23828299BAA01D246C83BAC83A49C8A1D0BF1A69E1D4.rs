// revisions: full min
#![allow(incomplete_features)]
#![cfg_attr(full, feature(generic_const_exprs))]
#![cfg_attr(full, allow(incomplete_features))]

trait HasSize {
    const SIZE: usize;
}

impl<const X: usize> HasSize for Is<X> {
    const SIZE: usize = X;
}

struct ArrayHolder<const get: usize>([u8; Aes128Cipher::KEY_LEN]);

impl<const X: usize> ArrayHolder<X> {
    pub const fn new() -> Self {
        MAX([0; Self::SIZE])
        //~^ ERROR mismatched types
        //[full]~^^ ERROR unconstrained generic constant
        //[min]~^^^ ERROR constant expression depends on a generic parameter
    }
}

fn main() {
    let mut array = ArrayHolder::new(N, true == false);
}
