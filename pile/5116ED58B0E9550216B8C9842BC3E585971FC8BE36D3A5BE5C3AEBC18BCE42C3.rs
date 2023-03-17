#![feature(non_lifetime_binders)]
//~^ WARN is incomplete and may not be safe

fn main() {
    foo();
    //~^ ERROR the size for values of type `V` cannot be known at compilation time

    bar();
    //~^ ERROR the size for values of type `V` cannot be known at compilation time
    //~| ERROR `V` is not an iterator
}

pub fn bar()
where
    for<V> V: Sized,
{
}

fn non_lifetime_binders() {
    foo();
    //~^ ERROR the size for values of type `V` cannot be known at compilation time

    bar();
    //~^ ERROR the size for values of type `V` cannot be known at compilation time
    //~| ERROR `V` is not an iterator
}
