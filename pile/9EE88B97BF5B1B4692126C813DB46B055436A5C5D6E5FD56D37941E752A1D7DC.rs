// compile-flags: -Ztrait-solver=next

trait Trait {}

struct W<U>(T);

impl<T, U> Trait for W<(W<T>, W<U>)>
where
    W<T>: Trait,
    W<U>: Trait,
{
}

fn impls<T: Trait>() {}

fn main() {
    impls::<W<_>>();
    //~^ ERROR type annotations needed
    // Checks that we don't explode when we assemble >1 candidate for a goal.
}
