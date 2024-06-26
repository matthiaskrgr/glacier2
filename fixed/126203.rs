#![feature(generic_const_exprs, generic_arg_infer)]

use core::mem::MaybeUninit;

pub struct Arr<T, const N: usize> {
    v: [MaybeUninit<T>; N],
}

impl<T, const N: usize> Arr<T, N> {
    const ELEM: MaybeUninit<T> = MaybeUninit::uninit();
    const INIT: [MaybeUninit<T>; N] = [Self::ELEM; N];

    fn new() -> Self {
        Arr { v: Self::INIT }
    }
}

pub struct BaFormatFilter<const N: usize> {}

pub enum DigitalFilter<const N: usize>
where
    [(); N * 2 + 1]: Sized,
    [(); N * 2]: Sized,
{
    Ba(BaFormatFilter<{ N * 2 + 1 }>),
}

pub fn iirfilter_st_copy<const N: usize, const M: usize>(_: [f32; M]) -> DigitalFilter<N>
where
    [(); N * 2 + 1]: Sized,
    [(); N * 2]: Sized,
{
    let zpk = zpk2tf_st(&Arr::<f32, { N * 2 }>::new(), &Arr::<f32, { N * 2 }>::new());
    DigitalFilter::Ba(zpk)
}

pub fn zpk2tf_st<const N: usize>(_z: &Arr<f32, N>, _p: &Arr<f32, N>) -> BaFormatFilter<{ N + 1 }> {
    BaFormatFilter {}
}

fn main() {
    iirfilter_st_copy::<4, 2>([10., 50.]);
}
