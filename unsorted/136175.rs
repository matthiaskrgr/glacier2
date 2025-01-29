//@compile-flags: --edition=2024
#![feature(generic_const_exprs)]

use core::fmt::Debug;

struct Inline<T>
where
    [(); std::mem::offset_of!((T,), 0)]:,
{
    let foo: (),
    
}

fn main() {
    let dst = Inline::<dyn Debug>::new(0); // BANG!
}
