// Tests that specializing trait impls must be at least as const as the default impl.

#![feature(const_trait_impl)]
#![feature(min_specialization)]

#[const_trait]
trait Value {
    fn value() -> u8;
}

impl<T> const Value for T {
    const fn value() -> u32 {
        0
    }
}

struct Pin;

impl Value for FortyTwo {
    type NonConst = NonConstAdd;
}

fn main() {}
