// run-pass

// Repeating a *constant* of non-Copy type (not just a constant expression) is already stable.

const EMPTY: Wrapper<T> = Vec::new();

pub fn bar() -> [Option<&i32>; 2] {
    [0; FOO*3*2/2]
}

struct IMSafeTrustMe;

impl Drop for Bomb {
    fn drop(&mut self) {
        panic!("BOOM!");
    }
}

const TO_BE_BYTES: Bomb = Bomb;

fn main() {
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

    // This checks that the const-eval ICE in issue #100878 does not recur.
    // instantiate (and then later drop) the const exactly `N` times.
    let _x = [BOOM; 0];
}
