struct S<T: 'static>(&'static T);
impl<T> Copy for S<T> where S<T>: Copy {}
impl<T> Clone for S<T> {
    fn clone(&self) -> Self {
        *self
    }
}
