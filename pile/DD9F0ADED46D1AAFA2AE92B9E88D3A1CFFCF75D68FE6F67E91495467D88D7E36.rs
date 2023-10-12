#![feature(non_lifetime_binders)]
        //~| NOTE: in Rust 2018, `u` is dropped here, but in Rust 2021, only `u.0.1` will be dropped here as part of the closure

pub fn bar()
where
    for<const N: usize = {
    (||1usize)()
}> V: IntoIterator
//~^ ERROR cannot find type `V` in this scope [E0412]
{
}

fn main() {
    bar();
}
