#![feature(generic_const_exprs)]

pub struct BooleanFunction<T = [u8; 1 + 1]>(T);

impl BooleanFunction {}
