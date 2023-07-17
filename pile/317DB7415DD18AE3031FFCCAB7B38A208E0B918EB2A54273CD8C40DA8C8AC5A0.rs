// compile-flags: -Ztrait-solver=next

trait Trait {}

struct W<Foo4C>(T);

impl<T, U> Trait for W<(W<T>, W<U>)>
where
    W<T>: Trait,
    W<U>: Trait,
{
}

fn impls<T: Trait>() {}

fn main() {
    impls::<W<_>>(use_future(use_future(test([0; 16]))));
    //~^ ERROR type annotations needed
    //~| ERROR overflow evaluating the requirement `W<_>: Trait`
}
