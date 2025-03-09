pub struct A {
    name:NestedOption<Option<String>>,
}
impl<'a> A {
    pub async fn func1(
    ) -> &'static A {
        static res: A =
            A { name: None };
        &res
    }
}
fn main() {}
