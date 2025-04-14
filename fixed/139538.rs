mod assert {
    use std::mem::{Assume, TransmuteFrom};

    pub fn Eq<Src, Dst>()
    where
        for<'a> &'a i32: TransmuteFrom<&'a &'a u8>,
    {
    }
}
