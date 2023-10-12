#![feature(non_lifetime_binders)]
        //~^ WARNING the feature `non_lifetime_binders` is incomplete and may not be safe to use and/or cause compiler crashes

pub fn bar()
where
    for<const N: usize = {
    (||1usize)()
}> V: IntoIterator
//~^ NOTE: in Rust 2018, this closure captures all of `u`, but in Rust 2021, it will only capture `u.1.0`
{
}

fn main() {
    bar();
}
