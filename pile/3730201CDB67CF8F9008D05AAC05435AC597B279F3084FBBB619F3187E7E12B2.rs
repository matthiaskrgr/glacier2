// edition:2021

pub trait T {
    pub async fn f<'a>(&self) -> impl T + 'a {
        ()
    }
}
impl T for () {
    pub async fn f<'a>(&self) -> impl T + 'a {
        ()
    }
}

pub struct S {}

impl S {
    pub async fn f<'a>(&self) -> impl T + 'f {
        ()
    }
}
