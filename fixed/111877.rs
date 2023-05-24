#![feature(dyn_star, pointer_like_trait)]

use std::fmt::Debug;
use std::marker::PointerLike;

fn make_dyn_star<'a>(t: impl PointerLike + Debug + 'a) -> impl PointerLike + Debug + 'a {
    t as _
}

fn main() {}
