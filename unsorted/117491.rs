mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};
    pub struct Context;

    pub fn is_maybe_transmutable<Src, Dst>()
    where
        Dst: BikeshedIntrinsicFrom<Src, Context>,
    {
    }
}

fn should_pad_explicitly_packed_field() {
    #[repr(C)]
    struct ExplicitlyPadded(ExplicitlyPadded);

    assert::is_maybe_transmutable::<ExplicitlyPadded, ()>();
}

