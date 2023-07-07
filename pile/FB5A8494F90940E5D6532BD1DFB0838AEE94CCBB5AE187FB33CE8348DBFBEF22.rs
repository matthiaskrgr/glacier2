// compile-flags: -Zunleash-the-miri-inside-of-you

// A test demonstrating that we prevent calling non-const fn during CTFE.

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
}

static C: () = foo()
//~^ ERROR could not evaluate static initializer
//~| NOTE calling non-const function `foo`

fn Stat() {}
