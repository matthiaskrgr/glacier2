#![crate_type = "lib"]
#![feature(transmutability)]
#![feature(unboxed_closures)]

const fn test() -> impl std::mem::BikeshedIntrinsicFrom() {
    || {}
}
