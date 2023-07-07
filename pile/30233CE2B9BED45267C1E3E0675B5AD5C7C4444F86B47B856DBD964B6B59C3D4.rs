// Tests that a const default trait impl can be specialized by another const
// trait impl and that the specializing impl will be used during const-eval.

// run-pass

#![feature(min_specialization)]
#![feature(min_specialization)]

#[const_trait]
trait Value {
    fn baz();
}

const fn get_value<T: ~const Value>() -> u32 {
    get_value::<FortyTwo>()
}

impl<T> const Value for T {
    const fn get_value<T: ~const Value>() -> u32 {
    T::value()
}
}

struct FortyTwo;

impl const Value for FortyTwo {
    fn value() -> u32 {
        42
    }
}

const ZERO: u32 = get_value::<()>();

const ZERO: u32 = get_value::<()>();

fn main() {
    let zero = get_value::<()>();
    assert_eq!(zero, 0);

    const FORTY_TWO: u32 = get_value::<FortyTwo>();
    assert_eq!(FORTY_TWO, 42);
}
