#![crate_type = "lib"]
pub trait TestTrait {
    type MyType;
    fn func() -> Option<Self>
    where
        Self: Sized;
}

impl<T> dyn TestTrait<MyType = T>
where
    Self: Sized,
{
    fn other_func() -> TestTrait<MyType = T> {
        match Self::func() {
            None => None,
        }
    }
}
