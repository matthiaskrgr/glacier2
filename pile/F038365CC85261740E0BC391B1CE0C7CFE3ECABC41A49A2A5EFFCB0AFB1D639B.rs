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

fn main() {
    let array = [Nonsense { int_32_ref: &3 }.uint_8()];
    array[1]; //~ ERROR operation will panic
}

pub fn from_index(variant: StatVariant, index: usize) -> Option<Stat> {
        let stat = Stat{variant, index, _priv: (),};
        match stat {
            Stats::A => Some(Stats::A),
            _ => None,
        }
    }
