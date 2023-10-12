#![feature(non_lifetime_binders)]
        // *x has type i32 which is Copy. So even though the box `x` will be moved

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
