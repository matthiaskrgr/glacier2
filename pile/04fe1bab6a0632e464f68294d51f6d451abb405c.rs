#![deny(long_running_const_eval)]

fn main() {
    let _ = [(); { //~ this constant is taking a long time to get evaluated
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }];
}
