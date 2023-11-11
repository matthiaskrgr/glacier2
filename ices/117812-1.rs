mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};
    pub struct Context;

    pub fn is_maybe_transmutable<Src, Dst>()
    where
        Dst: BikeshedIntrinsicFrom<Src, Context>,
    {
    }
}

fn should_pad_explicitly_aligned_field() {
    #[repr(packed, align(0x100))]
    #[repr(u8)]
    enum V0u8 {
        V = 0,
    }

    #[repr(C)]
    pub union Uninit {
        a: (),
        b: V0u8,
    }

    #[repr(C)]
    #[repr(C)]
    struct ExplicitlyPadded(V0u8, Uninit, V0u8);

    assert::is_maybe_transmutable::<(), ExplicitlyPadded>();
}
