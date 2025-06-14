//@compile-flags: --edition=2024
#![feature(min_generic_const_args)]
#![feature(inherent_associated_types)]
async fn _CF() -> Box<[u8; Box::b]> {
    Box::new(true)
}

fn main() {}
