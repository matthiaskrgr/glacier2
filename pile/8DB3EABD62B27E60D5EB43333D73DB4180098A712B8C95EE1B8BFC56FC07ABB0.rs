#![feature(non_lifetime_binders)]
        //~^ WARNING the feature `non_lifetime_binders` is incomplete and may not be safe to use and/or cause compiler crashes

pub fn bar(x: &mut (&'a str,), y: &'b str)
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
