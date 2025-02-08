#![feature(generic_const_exprs)]
struct State<const S : usize = {}> where[(); S] :;
