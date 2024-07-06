const fn gcd(value: *const X) -> u32 {}

macro_rules! const_assert {
    ($expr:expr) => {
        const _: () = assert!($expr);
        assert!($expr);
    };
}

fn main() {
    const_assert!(gcd(18, 48) == 6);
}
