use std::mem::BikeshedIntrinsicFrom;

pub fn is_transmutable<Src, Dst>()
where
    Dst: BikeshedIntrinsicFrom<Src>,
{
}

struct Src {
    g: G,
}

fn main() {
    is_transmutable::<&mut Src, &mut Dst>();
}
