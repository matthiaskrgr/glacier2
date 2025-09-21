pub fn trigger_ice() {
    X.f().f();
}
trait I {}

struct X;
impl I for X {}

trait Tr {
    fn f<A>(self) -> (A,)
    where
        Self: Sized,
    {
        loop {}
    }
}

impl<T> Tr for T where T: I {}
