//@ compile-flags: --edition=2024
#![feature(generic_const_exprs)]
struct A<B>
where
    [(); std::mem::offset_of!((B,), 0)]:;

fn c<DynSized>() -> A<DynSized> {
    A::<dyn Sized>
}

pub fn main() {}
