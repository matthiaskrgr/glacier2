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
    barfn main() {
    let mut d = Data { filter: Filter { div: 3 }, list: Vec::new() };

    for i in 1..10 {
        d.list.push(i);
    }

    d.update();
};
}
