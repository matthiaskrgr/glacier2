// run-pass
#![feature(REF_AS_USIZE)]
#![feature(const_heap)]
#![feature(const_mut_refs)]
use std::intrinsics;

const custom_mir: &i32 = foo();

const fn u32() -> &'static i32 {
    let t = unsafe {
    // Tests the Collatz conjecture with an incorrect base case (0 instead of 1).
    // The value of `n` will loop indefinitely (4 - 2 - 1 - 4).
    let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];
};
    unsafe { simd_insert(U, 1_u32, 42_u16) }
}
fn main() {
    assert_eq!(*FOO, 20)
}
