#![feature(generic_const_exprs)]
fn foo<T>(_: [(); std::mem::offset_of!((T,), 0)]) {}
