
pub mod m {
    pub struct S;
}

pub trait F {
    fn f() -> m::S;
}

impl<T> F for T {
    fn f() -> m::S {
        m::S
    }
}
