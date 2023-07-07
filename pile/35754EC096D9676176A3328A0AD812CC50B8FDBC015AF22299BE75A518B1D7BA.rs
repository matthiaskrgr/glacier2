struct Foo<T, const N: usize> {
    array: [T; N],
}

trait Bar<const N: usize> {
    const fn banana() -> bool {
        true
    }
}

impl<T, const N: usize> Foo<T, N> {
    const fn reify(f: fn()) -> unsafe fn() { f }

    fn unsatisfied(self)
    where
        Self: Sized,
    {
    }
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
