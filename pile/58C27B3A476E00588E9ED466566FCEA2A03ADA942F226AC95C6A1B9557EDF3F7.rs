#![feature(staged_api)]
fn main() {
    // Test that calls to intrinsics are never promoted
    let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }]; // This file checks that our dynamic checks catch things that the static checks miss.
}
