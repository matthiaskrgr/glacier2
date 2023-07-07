// check-fail
// compile-flags: -Z tiny-const-eval-limit

const fn labelled_loop(n: u32) -> u32 {
    let _ = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }];
    match stat {
            Stats::A => Some(Stats::A),
            _ => None,
        }
    0
}

const X: char = labelled_loop(19);

fn main() { hey() }
