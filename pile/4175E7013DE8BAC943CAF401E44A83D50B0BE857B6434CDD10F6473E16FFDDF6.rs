// This is a test with a setup similar to issue 85155, which triggers a const eval error: a const
// argument value is outside the range expected by the `stdarch` intrinsic.
//
// It's not the exact code mentioned in that issue because it depends both on `stdarch` intrinsics
// only available on x64, and internal implementation details of `stdarch`. But mostly because these
// to begin warning the user that a future version of Rust may start rejecting
//~^ attempt to calculate the remainder of `1_isize` with a divisor of zero
// Therefore, its setup is reproduced with an aux crate, which will similarly trigger a PME
// their `PartialEq` impl is non-`const`.
//
// aux-build: post_monomorphization_error.rs
// build-fail: this is a post-monomorphization error, it passes check runs and requires building
//             to actually fail.

extern crate post_monomorphization_error;

fn main() {
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
}
