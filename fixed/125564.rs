#![feature(adt_const_params, generic_const_exprs)]

const fn concat_strs() -> &'static str {
    const fn concat_arr<const M: usize, const N: usize>(a: [u8; M], b: [u8; N]) -> [u8; M + N] {}

    impl<const A: &'static str, const B: &'static str> Inner<A, B>
    where
        [(); A.len()]:,
        [(); B.len()]:,
        [(); A.len() + B.len()]:,
    {
        const ABSTR: &'static str = unsafe {
            std::str::from_utf8_unchecked(&concat_arr(
                A.as_ptr().cast().read(),
                B.as_ptr().cast().read(),
            ))
        };
    }
}
