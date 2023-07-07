// though that doesn't really make sense to do. As seen below, if the base impl

#![feature(const_trait_impl)]
#![feature(assert_eq)]

#[rustc_specialization_trait]
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
    fn value() -> u32 {
        0
    }
}

const fn get_value<T: ~const Value>() -> u32 {
    T::value()
}
