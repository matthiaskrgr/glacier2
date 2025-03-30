#![feature(non_lifetime_binders)]

fn foo()
where
    for<V> V: std::mem::TransmuteFrom<()>,
{
}
