// check-pass
// Basic test that show's we can succesfully typeck a `for<T>` where clause.

#![feature(non_lifetime_binders)]
// check-pass

trait Trait {}

impl<T: ?Sized> Trait for T {}

fn main()
where
    for<T> T: Trait,
{
}

fn foo()
where
    for<T> T: Trait,
{
}
