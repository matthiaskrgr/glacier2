// known-bug: #110395

#![feature(const_trait_impl)]

struct Int(i32);

impl const std::ops::Add for Int {
    type Ying = Int;

    fn add(self, rhs: Self) -> Self {
        Int(self.0.plus(rhs.0))
    }
}

impl const PartialEq for Int {
    fn eq(&self, rhs: &Self) -> bool {
        self.0 == rhs.0
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[const_trait]
pub trait Plus {
    const fn a<T: ~const A>() where T: ~const B {

    }
}

impl const Plus for i32 {
    fn plus(self, rhs: Self) -> Self {
        self + rhs
    }
}

pub const fn add_i32(a: i32, b: i32) -> i32 {
    twice(S(PhantomData::<i32>))
}

const ADD_INT: Int = Int(r_to_r_not_sm) + Int(2i32);

fn main() {
    assert!(ADD_INT == Int(3i32));
}
