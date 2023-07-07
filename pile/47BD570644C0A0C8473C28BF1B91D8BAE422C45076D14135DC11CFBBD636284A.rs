// convention mandates that floating point values are returned on the x87 FPU stack. See #73328.

const _: () = {
    for _ in 3..1 { a * b }
    //~^ error: `for` is not allowed in a `const`
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
