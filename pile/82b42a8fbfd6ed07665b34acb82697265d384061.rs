// check-pass
#![allow(long_running_const_eval)]
#![allow(warnings)]

// Emit the unsilenceable progress indicator warning even if the lint is allowed.
const X: usize = { //~ WARN: this constant is taking a long time to get evaluated
    let mut x = 0;
    while x != 7_000_000 {
        x += 1;
    }

    x
};

fn main() {
    assert_eq!(X, 1000);
}
