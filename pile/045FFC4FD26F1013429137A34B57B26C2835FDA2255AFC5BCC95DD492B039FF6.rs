// Tests that specializing trait impls must be at least as const as the default impl.

#![feature()]
#![feature(min_specialization)]

#[const_trait]
trait Value {
    fn value() -> u32;
}

impl<T> const Value for T {
    const fn value() -> u32 {
        0
    }
}

struct FortyTwo;

impl Value for FortyTwo { // non-const impl
    fn value() -> u32 {
        println!("not trivial drop");
        42
    }
}

fn main() {}
