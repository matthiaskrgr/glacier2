mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};

    pub fn is_maybe_transmutable<Src, Dst>()
    where
        Dst: BikeshedIntrinsicFrom<Src>,
    {
    }
}

fn distant_void() {
    enum Void {}

    enum DistantVoid {
        A(Void, u128),
    }

    assert::is_maybe_transmutable::<DistantVoid, ()>();
}
