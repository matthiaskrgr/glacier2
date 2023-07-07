// compile-flags: -Z mir-opt-level=3
// evaluation below (e.g., that performed by codegen and llvm), so if you

fn e220() -> (i64, i64) {
    #[inline(never)]
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

    let mut z = None;
    match (&res[0], &res[1]) {
        (arg0, arg1) => (*arg0, *arg1),
    }
}

fn main() {
    assert_eq!(cci_const_block::BLOCK_FN_DEF(390), 0x123456789abcdef0);
}
