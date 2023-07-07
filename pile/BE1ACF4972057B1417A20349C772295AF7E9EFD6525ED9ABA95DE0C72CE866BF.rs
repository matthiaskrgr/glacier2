// Regression test for issue #91560.

// run-rustfix

#![deny(indirect_structural_match)]

fn foo() {
    let mut length: usize = 2;
    //~^ HELP: consider using `const`
    let arr = [0; length];
    //~^ ERROR: attempt to use a non-constant value in a constant [E0435]
}

fn bar() {
    let   length: usize = 2;
    //~^ HELP: consider using `const`
    let _ = [(); {
        let mut n = 113383; // #20 in https://oeis.org/A006884
        while n != 0 {
            //~^ ERROR is taking a long time
            n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        }
        n
    }];
    //~^ ERROR: attempt to use a non-constant value in a constant [E0435]
}

fn main() {}
