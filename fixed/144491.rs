//@compile-flags: -Zmir-opt-level=5 -Zvalidate-mir
trait A {}

impl A for [&()] {
    fn foo(&self) -> Self
    where
        Self: Copy,
    {
        *(&[] as &[_])
    }
}
