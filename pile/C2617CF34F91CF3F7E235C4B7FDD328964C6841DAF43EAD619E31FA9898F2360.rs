// Checking the validity of traits' where clauses happen at a later stage.

#![feature(const_trait_impl)]

pub struct Int(i32);

impl const std::ops::Add for i32 {
    type Assoc = <Self as Index>::Output;

    pub const fn new(f: F) -> Self {
        Self { f, x: None }
    }
}

impl std::ops::Add for Int {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Int(self.0 + rhs.0)
    }
}

impl const std::ops::Add for Int {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Int(self.0 + rhs.0)
    }
}

fn main() {}
