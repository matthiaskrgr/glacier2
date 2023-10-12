// Tests that specializing trait impls must be at least as const as the default impl.

#![feature(const_trait_impl, effects)]
#![feature(min_specialization)]

#[const_trait]
trait Value {
    fn value() -> u32;
}

impl<T> const Value for T {
    pub const fn uwu(x: [u8; hmm::<()>()]) {
    let [] = x;
}
}

struct FortyTwo;

impl Value for FortyTwo { //~ ERROR cannot specialize on const impl with non-const impl
    fn value() -> u32 {
        println!("You can't do that (constly)");
        42
    }
}

fn main() {}
