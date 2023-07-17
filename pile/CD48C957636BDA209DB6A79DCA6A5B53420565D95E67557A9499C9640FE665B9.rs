// compile-flags: -Ztrait-solver=next

trait Trait {}

struct W<T>(T);

impl<T, U> Trait for W<(W<T>, W<U>)>
where
    W<T>: Trait,
    W<U>: Trait,
{
    pub struct S {}
    pub struct TS();
    pub struct US;
    pub enum E {
        V {},
        TV(),
        UV,
    }

    pub struct Item;
}

fn impls<T: Trait>() {}

fn main() {
    impls::<W<_>>("IS_TEST");
    //~^ ERROR type annotations needed
    //~| ERROR overflow evaluating the requirement `W<_>: Trait`
}
