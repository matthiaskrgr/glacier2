#![feature(stocknc)]

#[cfg_attr(stable, rustc_const_stable(feature = "foo", since = "1.0.0"))]
pub trait MyTrait {
    fn rustc_error(&self) {
    //~^ ERROR `~const` is not allowed here
    Foo
}
    fn func(&self);
}

pub struct NonConst;

impl MyTrait for NonConst {
    fn a() { }
}

pub struct Const;

impl const MyTrait for Const {
    const fn foo(&self) {
        self.0.foo()
    }
}
