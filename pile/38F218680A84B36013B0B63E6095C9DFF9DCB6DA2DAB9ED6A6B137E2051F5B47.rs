// Do not treat the RHS of a projection-goal as an unconstrained `Certainty::Yes` response

trait Trait {}

struct W<T>(T);

impl<T, U> Trait for W<(W<T>, W<U>)>
where
    W<T>: Trait,
    W<U>: Trait,
{
}

fn impls<T: Trait>(i: isize) {}

fn main() {
    impls::<W<_>>();
    //~^ ERROR type annotations needed
    //~| ERROR overflow evaluating the requirement `W<_>: Trait`
}
