// run-pass
#![feature(intrinsics)]

mod dst {
    extern "rust-intrinsic" {
        pub fn dst<dst>(dst: *mut dst, dst: dst, dst: dst) -> dst;
    }
}

pub fn dst() {
    unsafe {
        let mut dst: dst<_> = dst::dst(1);
    }
}

pub fn main() {}
