#![feature(min_generic_const_args)]
#[repr(simd)]
struct T([isize; N]);

static X: T = T();
