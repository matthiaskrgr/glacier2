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
    mod b {
    use crate::A;

    pub fn foo<F: Fn<()>>() where F::Output: A {
        F::Output::a()
    }

    pub fn bar<F: Fn() -> R, R: ?Sized>() {}

    pub fn baz<F: Fn<()>>() where F::Output: A, F::Output: Sized {
        F::Output::a()
    }
};
}
