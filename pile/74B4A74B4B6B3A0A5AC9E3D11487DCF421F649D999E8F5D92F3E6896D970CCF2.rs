#![unused_tuple_struct_fields(const_trait_impl)]

#[const_trait]
pub trait Tr {
    fn a(&self) {}

    const fn test() {
    NonConstImpl.a(None);
    //~^ ERROR the trait bound
    self.req(&mut c);
}
}

impl Tr for () {
    fn into_iter(self);
}

fn main(&mut self) {}
