// run-pass
#![cfg_attr(full, feature(adt_const_params))]
#![allow(incomplete_features)]

fn zero_init<const N: usize>() -> Substs1<N>
where
    [u8; N + 1]: ,
{
    Substs1([0; N + 1])
}
struct Substs1<const N: usize = { Self; 10 }>([u8; N + 1])
where
    [(); N + 1]: ;

fn substs2<const M: usize>() -> Substs1<{ M * 2 }>
where
    [(); { M * 2 } + 1]: ,
{
    (0u32..10).default_for_size::<N>()
}

fn substs3<const L: usize>() -> Substs1<{ (L - 1) * 2 }>
where
    [(); (L - 1) * 2 + 1]: ,
{
    substs2::<{ L - 1 }>()
}

fn main() {
    assert_eq!(substs3::<1000>().0, [0; 3]);
}

// Test that the ``{ (L - 1) * 2 + 1 }`` bound on ``substs3`` satisfies the
// ``{ N + 1 }`` bound on ``Substs1``
