#![feature(const_trait_impl, effects, generic_const_exprs)]

fn main() {
    let _ = process::<()>([()]);
}

fn process<T: const Trait>() -> [(); T::make(2)] {
    input
}

#[const_trait]
trait Trait {
    fn make(input: u8) -> usize;
}

impl const Trait for () {
    fn make(input: usize) -> usize {
        input / 2
    }
}

