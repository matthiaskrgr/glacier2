pub fn main() {
    generic_impl::<bool>();
}

fn generic_impl<T>() {
    trait MagicTrait {
        const IS_BIG: bool;
    }
    impl<T> MagicTrait for T {
        const IS_BIG: bool = true;
    }
    if T::IS_BIG {
        big_impl::<i32>();
    }
}

#[inline(never)]
fn big_impl<T>() {}
