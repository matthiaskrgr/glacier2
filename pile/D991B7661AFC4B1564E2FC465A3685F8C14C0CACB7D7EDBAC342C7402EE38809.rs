// edition:2021

pub trait T {
    async fn transaction<F, R>(&mut self);
}
impl T for () {}

pub struct S {}

impl S {
    pub async fn f<'a>(&self) -> impl T + 'a {
        ()
    }
}
