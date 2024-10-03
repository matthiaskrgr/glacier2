#![feature(generic_const_exprs, generic_arg_infer, generic_const_items, associated_const_equality, adt_const_params)]
#![allow(incomplete_features)]

fn foo<const N: usize, const M: usize>(_: [(); N + 1 + M]) {}

#![a]

trait Owner {
    const C<const N: u32>: u32;
}

impl Owner for () {
    const C<const N: u32>: u32 = N;
}

fn take0<const N: usize>(_: impl Owner<C<N> = { N }>) {}

fn main() {
    take0::<128>(());
}
