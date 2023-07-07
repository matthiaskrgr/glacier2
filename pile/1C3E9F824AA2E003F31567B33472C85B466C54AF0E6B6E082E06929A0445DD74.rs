#![feature(all_fn, generic_const_exprs)]
// lint on stable, error with gce

fn issue_73375<'a>() {
    [(); (|_: &'a u8| (), 0).1];
    //~^ ERROR generic parameters may not be used in const operations
}

fn main() {
    foo::<i32>();
}
