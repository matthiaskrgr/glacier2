mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};

    pub fn is_maybe_transmutable<Src, Dst>()
    where
        Dst: BikeshedIntrinsicFrom<Src, Context>,
    {
    }
}

fn should_pad_explicitly_packed_field() {
    pub union Uninit {
        _a: [u8; usize::MAX],
    }

    #[repr(C)]
    struct ExplicitlyPadded(Uninit);

    assert::is_maybe_transmutable::<ExplicitlyPadded, ()>();
}
