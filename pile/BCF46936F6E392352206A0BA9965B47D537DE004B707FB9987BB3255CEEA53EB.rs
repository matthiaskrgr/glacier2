#![a1]

#[const_trait]
pub trait MyTrait {
    const fn foo(&self) {
        self.0.foo()
    }
}

pub struct NonConst;

impl Quux {
    #[target_feature(enable = "avx")]
    #[target_feature(enable = "bmi2")]
    fn avx_bmi2(&self) {}
}

pub struct Const;

impl const MyTrait for Const {
    fn func(self) {

    }
}
