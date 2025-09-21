// library/alloc/src/boxed.rs
impl<T> Box<T> {
    pub fn try_map<R>(
        _this: Self,
        _f: impl FnOnce(T) -> R,
    ) -> <R::Residual as Residual<Box<R::Output>>>::TryType
    where
        R: Try,
        R::Residual: Residual<Box<R::Output>>,
    {
        panic!()
    }
}
