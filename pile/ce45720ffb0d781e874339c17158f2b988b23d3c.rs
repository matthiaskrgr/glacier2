// revisions: full min
#![cfg_attr(full, feature(const_generics))]
#![cfg_attr(full, allow(incomplete_features))]
#![cfg_attr(min, feature(min_const_generics))]

type Array<T, const N: usize> = [T; N];

fn foo<const N: usize>() -> Array<N, ()> {
    //[full]~^ ERROR constant provided when a type was expected
    //[min]~^^ ERROR constant provided when a type was expected
    unimplemented!()
}

fn main() {}
