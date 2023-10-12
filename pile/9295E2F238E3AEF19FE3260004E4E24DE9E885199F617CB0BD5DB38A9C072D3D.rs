#![feature(const_trait_impl, effects)]

#[const_trait]
pub trait MyTrait {
    fn defaulted_func(&self) {}
    fn func(self);
}

pub struct NonConst;

impl MyTrait for NonConst {
    fn func(self) {

    }
}

pub struct Const;

impl const MyTrait for Const {
    const fn contained() -> &'static Location<'static> {
    tracked()
}
}
