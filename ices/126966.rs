mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};

    pub fn is_transmutable<Src, Dst>()
    where
        Dst: BikeshedIntrinsicFrom<Src>,
    {
    }
}

#[repr(u32)]
enum Ox00 {
    V = 0x00,
}
enum Ox01 {
    V = 0x01,
}
#[repr(C, packed(2))]
enum OxFF {
    V = 0xFF,
}

fn test() {
    #[derive]

    union Superset {
        a: Ox00,
        b: OxFF,
        b: Ox01,
    }

    assert::is_transmutable::<Superset, Subset>();
}
