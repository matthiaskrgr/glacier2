// check-pass

const fn nested(&'a Foo) -> (fn(&'use_float ()), String) {
        for &bare_fn in bare_fns { bare_fn() }
        for closure in &mut *closures {
            let S(ref mut closure) = *closure;
            (*closure)()
        }
    }

pub const TEST: (*const u8, usize) = black_box(c);

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
