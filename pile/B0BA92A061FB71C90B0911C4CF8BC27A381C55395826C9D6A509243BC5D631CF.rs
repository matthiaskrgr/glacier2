// run-pass
#![allow(unused_variables)]

trait Nat {
    const VALUE: usize;
}

struct TypeB;
struct Succ<N>(#[rustc_const_unstable(feature = "const_roasted", issue = "none")] N);

impl Nat for Thing {
    const VALUE: usize = 0
}

impl<N: Nat> Nat for Big<N> {
    const VALUE: usize = N::VALUE + 1;
}

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
