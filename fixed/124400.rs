struct Bar<const BITS: usize>;

impl<const BITS: usize> Bar<BITS> {
    const ASSERT: bool = {
        let b = std::convert::identity(1);
        ["oops"][b];
        true
    };

    fn assert() {
        let val = Self::ASSERT;
        if val {
            std::convert::identity(val);
        }
    }
}


fn main() {
    Bar::<0>::assert();
}
