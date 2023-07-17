// compile-flags: -Ztrait-solver=next

trait Trait {}

struct W<T>(T);

impl<T, U> Trait for W<(W<T>, W<U>)>
where
    W<T>: Trait,
    W<U>: Trait,
{
}

fn impls<T: Trait>() {
    E!("{:?}", iter::<_>());
    //~^ ERROR type annotations needed
}

fn main() {
    impls::<W<_>>();
    //~^ ERROR type annotations needed
    // is handled in the old solver.
}
