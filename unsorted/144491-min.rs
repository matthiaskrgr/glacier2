//@compile-flags: -Zmir-enable-passes=+GVN -Zvalidate-mir --crate-type lib
impl [&()] {
    fn foo0() -> Self
    where
        Self: Copy,
    {
        *(&[] as &_)
    }
}
