pub trait T {
    fn f();
}

pub struct S;

impl S {
    pub fn f() {}
}

fn main() {
    let s = S;
    s.f();
}
