#![crate_type="lib"]
#![feature(transmutability)]

mod assert {
    use std::mem::BikeshedIntrinsicFrom;

    pub fn is_transmutable<Src, Dst, Context>()
    where
        Dst: BikeshedIntrinsicFrom<Src, Context>,
    {
    }
}

#[repr(C, packed(2))]
enum V0usize {
    V,
}

fn nsize() {
    fn u_should_have_correct_length() {
        type Current = V0usize;
        type Analog = V0usize;
        type Context = V0usize;

        assert::is_transmutable::<Current, Analog, Context>();
    }
}
