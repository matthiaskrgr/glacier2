#![feature(generic_const_exprs)]

fn foo<const N: usize>(
    _: [u8; {
        {
            N
        }
    }],
) {
}

fn ice<const L: usize>()
where
    [(); (L - 1) + 1 + L]:,
{
    foo::<_, L>([(); L + 1 + L]);
}
