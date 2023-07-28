// compile-flags: -Ztrait-solver=next

trait Trait {}

struct W<T>(T);

impl<T, U> Trait for W<(W<T>, W<U>)>
where
    W<T>: Trait,
    W<U>: Trait,
{
}

fn impls<T: Trait>() {}

fn main() {
    impls::<W<_>>(&1i16);
    //~^ ERROR type annotations needed
    //~| ERROR overflow evaluating the requirement `W<_>: Trait`
}
