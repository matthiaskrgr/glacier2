#![feature(non_lifetime_binders)]

fn auto_trait()
where
    for<T> T:PartialEq + PartialOrd,
{}
