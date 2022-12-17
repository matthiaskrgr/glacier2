#![allow(incomplete_features)]
#![feature(adt_const_params, const_ptr_read, generic_const_exprs)]

use std::mem::ManuallyDrop;

const fn concat_strs<const A: &'static str, const B: &'static str>() -> &'static str
where
    [(); A.len()]:,
    [(); B.len()]:,
    [(); A.len() + B.len()]:,
{
    #[repr(C)]
    struct ConcatJoin<const N: usize, const M: usize> {
        left: [u8; N],
        right: [u8; M],
    }

    #[repr(C)]
    union ConcatJoiner<const N: usize, const M: usize>
    where
        [(); N + M]:,
    {
        whole: ManuallyDrop<[u8; N + M]>,
        split: ManuallyDrop<ConcatJoin<N, M>>,
    }

    const fn concat_arr<const M: usize, const N: usize>(a: [u8; M], b: [u8; N]) -> [u8; M + N]
    where
        [(); M + N]:,
    {
        unsafe {
            let joiner = ConcatJoiner {
                split: ManuallyDrop::new(ConcatJoin { left: a, right: b }),
            };
            let join = joiner.whole;
            ManuallyDrop::into_inner(join)
        }
    }

    struct Inner<const A: &'static str, const B: &'static str>;
    impl<const A: &'static str, const B: &'static str> Inner<A, B>
    where
        [(); A.len()]:,
        [(); B.len()]:,
        [(); A.len() + B.len()]:,
    {
        const ABSTR: &'static str = unsafe {
            std::str::from_utf8_unchecked(&concat_arr(
                A.as_ptr().cast::<[u8; A.len()]>().read(),
                B.as_ptr().cast::<[u8; B.len()]>().read(),
            ))
        };
    }

    Inner::<A, B>::ABSTR
}

const FOO: &str = "foo";
const BAR: &str = "bar";
const FOOBAR: &str = concat_strs::<FOO, BAR>();

pub fn main() {}
