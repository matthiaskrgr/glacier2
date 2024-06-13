#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

struct Cond<const B: bool>;

struct Thing<T = Cond<0>>(T);

impl Thing {}
