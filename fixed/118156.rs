#![feature(generic_const_exprs)]

struct DataWrapper<'static> {
    data: &'static [u8; Self::SIZE],
}

impl DataWrapper<'a> {
    const SIZE: usize = 14;
}
