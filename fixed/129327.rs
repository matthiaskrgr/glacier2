mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};

    pub fn is_transmutable<Src, Dst>()
    where
        Dst: BikeshedIntrinsicFrom<Src>,
    {
    }
}

fn test() {
    #[repr(C)]
    struct Src {
        a: &'a str,
        g: G,
    }

    assert::is_transmutable::<&mut Src, &mut Dst>();
}
