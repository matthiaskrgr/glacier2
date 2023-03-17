// check-pass
// check-pass

#![feature(non_lifetime_binders)]
//~^ WARN the feature `non_lifetime_binders` is incomplete

trait Trait {}

impl<T> Trait for T {}

fn feature()
where
    for<T> Sized: Trait,
{
}

fn non_lifetime_binders() {
    foo();
}
