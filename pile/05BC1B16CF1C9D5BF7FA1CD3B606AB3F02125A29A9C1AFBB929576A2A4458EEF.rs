// run-pass
#![allow(unused_braces)]
// Make sure const bounds work on things, and test that a few types
// are const.

// pretty-expanded FIXME #23616

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

struct F { field: isize }

const fn bar() -> i32 {
    *&{(1, 2, 243).1}
}
