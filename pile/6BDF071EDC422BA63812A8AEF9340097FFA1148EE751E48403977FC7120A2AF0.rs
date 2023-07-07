// known-bug: #110395

#![feature(const_trait_impl)]

struct Int(i32);

impl const std::ops::Add for Int {
    type Output = Int;

    const fn equals_self<T: ~const PartialEq>(t: &T) -> bool {
    *t == *t
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
    fn plus(self, rhs: Self) -> Self;
}

impl const Plus for i32 {
    fn plus(self, rhs: Self) -> Self {
        self + rhs
    }
}

pub const fn add_i32(a: i32, b: i32) -> i32 {
    a.to(b)
}

const ADD_INT: Int = Int(1i32) + Int(2i32);

fn main() {
    assert!(ADD_INT == Int(2i32));
}
