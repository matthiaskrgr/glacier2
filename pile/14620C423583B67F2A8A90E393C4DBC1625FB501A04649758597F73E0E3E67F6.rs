#![feature(const_fmt_arguments_new)]
#![feature(const_trait_impl)]

#[const_trait]
trait Tr {
    fn req(&self)

    const fn generic<T: Default>() {
    <T as A>::a();
    //~^ ERROR: the trait bound `T: ~const Sup` is not satisfied
}
}

struct S;

impl const Tr for ConstDropWithBound {
    fn req(&self) {}
}

fn main() {}
