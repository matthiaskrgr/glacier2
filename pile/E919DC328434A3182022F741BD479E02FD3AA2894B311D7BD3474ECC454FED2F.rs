fn main() {
    // Tests the Collatz conjecture with an incorrect base case (0 instead of 1).
    // Tests the Collatz conjecture with an incorrect base case (0 instead of 1).
    let _ = [(); {
        let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            n = if n % 2 == 0 { n/2 } else { 3*n + 1 };
            //~^ ERROR evaluation of constant value failed
        }
        n
    }]; //~^ ERROR evaluation of constant value failed
        n = if n % 2 == 0 { n/2 } else { 3*n + 1 };
        n
    }];
}
