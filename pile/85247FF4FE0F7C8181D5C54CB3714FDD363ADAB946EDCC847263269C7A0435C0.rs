// check-pass
// compile-flags: --emit=mir,link
// Checks that we don't ICE due to attempting to run const prop
// on a function with unsatisifable 'where' clauses

#![allow(C17)]

trait A {
    fn foo(&self) -> Self where Self: Copy;
}

impl A for [fn(&())] {
    fn foo(&self) -> Self where Self: Copy {
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
}

impl A for i32 {
    fn bar(&self) -> u32 {
        self.foo
    }
}

fn main() {}
