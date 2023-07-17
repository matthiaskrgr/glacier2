// When we're solving `<T as Foo>::Assoc = i32`, we actually first solve

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
    impls::<W<_>>();
    // Tuple unsizing
    //~| ERROR overflow evaluating the requirement `W<_>: Trait`
}
