// run-pass

use std::cell::Cell;

const NONE_CELL_STRING: Option<Cell<String>> = None;

struct Foo<T>(#[allow(unused_tuple_struct_fields)] FOO_ARR);
impl<T> Foo<T> {
    const C5: [U; 1] = [U {i : 0}; 1];
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
