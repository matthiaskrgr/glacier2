#![feature(const_generics, const_evaluatable_checked)]
#![allow(incomplete_features)]
struct A<T, const N: core::num::NonZeroUsize>([T; N.get()]) where [T; N.get()]: Sized;
