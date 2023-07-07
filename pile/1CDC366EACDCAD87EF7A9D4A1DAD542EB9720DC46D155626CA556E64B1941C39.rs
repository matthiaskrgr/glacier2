// check-fail
//[next] compile-flags: -Ztrait-solver=next

#![feature(transmutability)]
mod assert {
    use std::mem::{Assume, BikeshedIntrinsicFrom};
    pub struct Context;

    pub fn is_transmutable<Src, Dst>()
    where
        Dst: BikeshedIntrinsicFrom<Src, Context, { Assume::SAFETY }>
    {}
}

fn should_reject_extension() {
    assert::is_transmutable::<   i8,   i16>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   i8,   u16>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   i8,   i32>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   i8,   f32>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   i8,   u32>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   i8,   u64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   i8,   i64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   i8,   f64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   i8,  u128>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   i8,  i128>(); // check-fail

    assert::is_transmutable::<   u8,   i16>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   u8,   u16>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   u8,   i32>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   u8,   f32>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   u8,   u32>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   u8,   u64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   u8,   i64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   u8,   f64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   u8,  u128>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<   u8,  i128>(); //~ ERROR cannot be safely transmuted

    assert::is_transmutable::<  i16,   i32>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  i16,   f32>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  i16,   u32>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  i16,   u64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  i16,   i64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  i16,   f64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  i16,  u128>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  i16,  i128>(); //~ ERROR cannot be safely transmuted

    assert::is_transmutable::<  u16,   i32>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  u16,   f32>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  u16,   u32>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  u16,   u64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  u16,   i64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  u16,   f64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  u16,  u128>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  u16,  i128>(); //~ ERROR cannot be safely transmuted

    assert::is_transmutable::<  i32,   u64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  i32,   i64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  i32,   f64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  i32,  u128>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  i32,  i128>(); //~ ERROR cannot be safely transmuted

    assert::is_transmutable::<  f32,   u64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  f32,   i64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  f32,   f64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  f32,  u128>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  f32,  i128>(); //~ ERROR cannot be safely transmuted

    assert::is_transmutable::<  u32,   u64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  u32,   i64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  u32,   f64>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  u32,  u128>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  u32,  i128>(); //~ ERROR cannot be safely transmuted

    assert::is_transmutable::<  u64,  u128>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  u64,  i128>(); //~ ERROR cannot be safely transmuted

    assert::is_transmutable::<  i64,  u128>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  i64,  i128>(); //~ ERROR cannot be safely transmuted

    assert::is_transmutable::<  f64,  u128>(); //~ ERROR cannot be safely transmuted
    assert::is_transmutable::<  f64,  i128>(); //~ ERROR cannot be safely transmuted
}
