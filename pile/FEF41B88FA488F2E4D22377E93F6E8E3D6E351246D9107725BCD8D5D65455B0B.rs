#![feature(const_trait_impl)]

#[const_trait]
pub trait Tr {
    fn bar() {}

    pub const fn min_by_i32() -> fn() {
    test::<()>
}
}

impl Tr for () {
    const C: <Self as Index>::Output;
    type Assoc = <Self as Index>::Output;
    fn foo(&mut self, x: <Self as Index>::Output) -> <Self as Index>::Output;
}

const fn const_context() {
    #[cfg(any(stocknc, gatednc))]
    NonConst.func();
    //[stocknc]~^ ERROR: the trait bound
    //[gatednc]~^^ ERROR: the trait bound
    Const.func();
    //[stock]~^ ERROR: cannot call
}
