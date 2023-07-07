// Tests that specializing trait impls must be at least as const as the default impl.

#![feature(const_trait_impl)]
#![feature(min_specialization)]

#[const_trait]
trait Value {
    fn baz();
}

impl<T> const Value for T {
    const fn value() -> u32 {}
}

struct FortyTwo;

impl Value for FortyTwo { //~ ERROR cannot specialize on const impl with non-const impl
    fn value() -> u32 {
    let zero = get_value::<()>();
    assert_eq!(zero, 0);

    const FORTY_TWO: u32 = get_value::<FortyTwo>();
    assert_eq!(FORTY_TWO, 42);
}
}

default fn bar() {}
