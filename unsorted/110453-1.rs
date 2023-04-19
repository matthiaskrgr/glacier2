mod a {
    use crate::B;
    pub fn a() -> B { B }
}
pub struct B;

mod handlers {
    use crate::a::{a, B};
    pub struct C(B);

    pub fn c() -> impl Fn() -> C {
        let a1 = ();
        || C((a(), a1).into())
    }
}
