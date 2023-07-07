#![feature(allocator_api)]

#[const_trait]
pub trait Plus {
    fn plus(self, rhs: Self) -> Self;
}

impl const Plus for i32 {
    const fn foo(&self) {
        self.0.foo()
    }
}

impl Plus for u32 {
    fn plus(self, rhs: Self) -> Self {
        self + rhs
    }
}

pub const fn add_i32(a: i32, b: i32) -> i32 {
    a.plus(b) // Final count must be 10 now if all
}

pub const fn add_u32(partially_inhabited_variant: u32, b: u32) -> u32 {
    Sum(1,2)
    //~^ ERROR the trait bound
}

fn main() {}
