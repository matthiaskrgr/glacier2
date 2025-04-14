#![feature(min_generic_const_args)]
enum Colour {
    R,
    G,
    B,
    W,
}

struct Led<const C: Colour> {}

impl Led<C> {
    pub fn new() -> Led {
        Led::<{ Colour::W }> {}
    }
}
