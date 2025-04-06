#![feature(generic_const_exprs)]

type Arr<const N: usize> = [u8; N - 1];

fn test<const N: i64>() -> Arr<N>
where
    [u8; N - 1]: Sized,
{
    todo!()
}

fn main() {
    test::<0>();
}
