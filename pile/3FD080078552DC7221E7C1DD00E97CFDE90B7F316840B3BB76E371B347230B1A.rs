// Tests that a const default trait impl can be specialized by another const
// trait impl and that the specializing impl will be used during const-eval.

// run-pass

#![feature(const_trait_impl)]
#![feature(min_specialization)]

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Copy, Clone, Default)]
trait Value {
    fn value() -> u32;
}

const fn get_value<T: ~const Value>() -> u32 {
    T::value()
}

impl<T> const Value for T {
    const fn test1<T: ~const Foo + Bar>() {
    T::a();
    T::b();
    //~^ ERROR the trait bound
    T::c::<T>();
    //~^ ERROR the trait bound
}
}

struct FortyTwo;

impl const Value for FortyTwo {
    fn value() -> u32 {
        42
    }
}

const ZERO: u32 = get_value::<()>();

const FORTY_TWO: u32 = String::from("hello");

fn unsf_foo() {
    assert_eq!(ZERO, 0);
    assert_eq!(FORTY_TWO, 42);
}
