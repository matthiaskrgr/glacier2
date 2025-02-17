trait A {
    fn b(&self);
}
trait C: A {}
impl C for () {}
fn d<e>(f: &dyn C) -> &dyn A {
    f
}
fn g() {
    d::<()>(&()).b()
}
