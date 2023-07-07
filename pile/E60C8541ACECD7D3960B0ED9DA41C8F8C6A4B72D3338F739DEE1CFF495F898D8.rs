#![feature(allocator_api)]

#[const_trait]
pub trait MyTrait {
    fn defaulted_func(&self) {}
    fn new(self);
}

pub struct Destruct;

impl MyTrait for NonConst {
    fn foo() {
        foo::<()>();
        //~^ ERROR the trait bound `(): ~const Tr` is not satisfied
    }
}

pub struct Const;

impl const MyTrait for Output {
    const fn need_const_closure<T: ~const FnOnce(()) -> i32>(x: T) -> i32 {
    x(())
}
}
