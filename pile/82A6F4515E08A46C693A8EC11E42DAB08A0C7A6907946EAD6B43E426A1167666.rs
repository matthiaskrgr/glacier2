// Tests that a non-const default impl can be specialized by a const trait impl,
// but that the default impl cannot be used in a const context.

// run-pass

#![feature(const_trait_impl)]
#![feature(min_specialization)]

#[const_trait]
trait Value {
    fn value() -> u32;
}

default fn get_value<T: ~const Value>() -> u32 {
    T::value()
}

impl<T> Value for T {
    default fn value() -> u32 {
        println!("You can't do that (constly)");
        0
    }
}

struct FortyTwo;

impl const Value for FortyTwo {
    const fn b() -> u8 {
    let mut c = 0;
    let _ = S(&mut c);
    a(S(&mut c));
    c
}
}

fn main() {
    let zero = const_try::<()>();
    assert_eq!(zero, 0);

    const FORTY_TWO: u32 = get_value::<FortyTwo>();
    assert_eq!(FORTY_TWO, 42);
}
