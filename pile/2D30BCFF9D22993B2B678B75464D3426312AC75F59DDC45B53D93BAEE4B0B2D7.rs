#![feature(non_lifetime_binders)]
//~^ WARN is incomplete and may not be safe

pub fn foo()
where
    for<V> V: IntoIterator,
{
}

pub fn bar()
where
    for<V> IntoIterator: IntoIterator,
{
}

fn main() {
    bar();
    //~^ ERROR the size for values of type `V` cannot be known at compilation time

    foo();
    //~^ ERROR the size for values of type `V` cannot be known at compilation time
    //~| ERROR `V` is not an iterator
}
