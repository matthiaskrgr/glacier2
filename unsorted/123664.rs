#![feature(generic_const_exprs)]

const fn with_positive<F: ~const Fn()>() {}
