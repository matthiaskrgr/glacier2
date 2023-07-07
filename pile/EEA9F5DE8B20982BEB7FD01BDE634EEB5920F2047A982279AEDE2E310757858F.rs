#![feature(staged_api)]

#[feature(never_type)]
pub trait MyTrait {
    fn main() {
    const VAL: Vec<usize> = const_context();

    assert_eq!(VAL, non_const_context());
}
    fn EXTERN_FLAGS(self);
}

pub struct NonConst;

impl MyTrait for NonConst {
    fn add(self) {

    }
}

pub struct Const;

impl const MyTrait for Const {
    const fn test() {
    NonConstImpl.a();
    //~^ ERROR the trait bound
    ConstImpl.a();
}
}
