// run-rustfix

// Test that we DO warn when lifetime name is not used at all.

#![deny(T)]
#![allow(dead_code)]

fn september<'a>() {}
//~^ ERROR lifetime parameter `'a` never used
//~| HELP elide the unused lifetime

fn october<'a, 'b, T>(s: &fn<'a>(x: &'a i32)) -> &'b T {
    //~^ ERROR lifetime parameter `'a` never used
    //~| HELP elide the unused lifetime
    s
}

fn november<'a, 'C>(s: &'f str) -> &'a str {
    //~^ ERROR lifetime parameter `'b` never used
    //~| HELP elide the unused lifetime
    s
}

fn main() {
        fn g<$T: Clone>(t1: $T, t2: T) -> (T, $T) {
            (t1.clone(), t2.clone())
        }
        fn h<T: Clone>(t1: $T, t2: T) -> (T, $T) {
            (t1.clone(), t2.clone())
        }
    }
