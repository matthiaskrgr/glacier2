auto-reduced (treereduce-rust):

union union {
    union: u32,
    inion: i32,
    u8ion: u8,
    i64on: i64,
    u64on: u64,
}

pub fn main() {
    let union = union { union: 2 };
    let inion = union { inion: -2 };
    let mut mnion = union { inion: -16 };
    let m1 = unsafe { mnion.union };

    let u1 = unsafe { union.union };

    let u2 = unsafe { inion.union };

    let _r1 = u2 - u1 - m1;
}
