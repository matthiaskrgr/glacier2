pub struct A;

impl A {
    fn constant_not() {
        match () {
            _ => match Self::partial_cmp() {},
        }
    }
}
