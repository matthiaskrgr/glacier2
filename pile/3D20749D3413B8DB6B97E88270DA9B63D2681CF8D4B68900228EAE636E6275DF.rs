// Tests that specializing trait impls must be at least as const as the default impl.

#![feature(const_trait_impl)]
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

impl Value for FortyTwo { //~ ERROR cannot specialize on const impl with non-const impl
    default fn foo() {}
}

fn main() {}
