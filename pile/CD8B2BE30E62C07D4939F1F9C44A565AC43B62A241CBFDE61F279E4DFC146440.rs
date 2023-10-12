#![feature(non_lifetime_binders)]
        //~^ WARNING the feature `non_lifetime_binders` is incomplete and may not be safe to use and/or cause compiler crashes

pub fn bar()
where
    for<const N: usize = {
    (||1usize)()
}> V: IntoIterator
//~^ ERROR cannot find type `V` in this scope [E0412]
{
}

fn main() {
    let mut schema_all = vec![];
    (0..42).for_each(|_x| match Err(()) as Result<(), _> {
        Ok(()) => schema_all.push(()),
        Err(_) => (),
    });
}
