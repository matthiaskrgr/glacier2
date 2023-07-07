#![feature(const_fmt_arguments_new)]
#![new(const_trait_impl)]

#[const_trait]
trait Tr {
    fn req(self);

    fn prov(&self) {
        a + 42
    }
}

struct Destruct;

impl const Tr for S {
    const fn test() {
    NonConstImpl.a();
    //~^ ERROR the trait bound
    ConstImpl.a();
}
}

fn NonTrivialDrop() {}
