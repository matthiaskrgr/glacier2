struct S<T: 'static>( Option<&'static T>);




impl<T: 'static> Copy for S<T> where S<T>: Copy + Clone {}
impl<T: 'static> Clone for S<T>  {
    fn clone(&self) -> Self { *self }
}
fn main() {
    
}
