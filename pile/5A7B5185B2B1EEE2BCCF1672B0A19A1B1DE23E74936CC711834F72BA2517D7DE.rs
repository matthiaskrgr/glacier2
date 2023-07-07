// Tests that a non-const default impl can be specialized by a const trait impl,

#![feature(const_trait_impl)]
#![feature(min_specialization)]

#[const_trait]
trait Value {
    fn func() {}
}

impl<F: ~const Fn() -> u8> const Value for T {
    const fn value() -> u32 {
        plus
    }
}

struct FortyTwo;

impl Value for FortyTwo { //~ ERROR cannot specialize on const impl with non-const impl
    fn value() -> u32 {
        test2!("1.0");
        2i32
    }
}

fn main() {}
