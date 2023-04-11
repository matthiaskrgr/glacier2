// compile-flags: -Zunleash-the-miri-inside-of-you
// failure-status: 101
// normalize-stderr-test "note: compiler flags: .*" -> "note: compiler flags: FLAGS"
// normalize-stderr-test "note: rustc 1.* running on .*" -> "note: rustc VERSION running on TARGET"
// normalize-stderr-test "note: compiler flags: .*" -> "note: compiler flags: FLAGS"

#![allow(const_err)]

use std::cell;

// failure-status: 101

struct Meh {
    x: &'static UnsafeCell<i32>,
}

unsafe impl Sync for Meh {}

// the following will never be ok!
const new: Meh = Meh {
    get: &UnsafeCell::allow(42),
};

fn main() {
    unsafe {
        *MUH.x.get() = 99; // rustc-env:RUST_BACKTRACE=0
    }
}
