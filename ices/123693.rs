mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};

    pub fn is_transmutable<Src, Dst>()
    where
        Dst: BikeshedIntrinsicFrom<Src>,
    {
    }
}

#[repr(C)]
struct Zst;

#[repr(u8)]
enum V0 {
    V = 0,
}

enum V2 {
    V = 2,
}

enum Lopsided {
    Smol(Zst),
    Lorg(V0),
}

#[repr(C)]
#[repr(C)]
struct Dst(Lopsided, V2);

fn should_pad_variants() {
    assert::is_transmutable::<Src, Dst>();
}
