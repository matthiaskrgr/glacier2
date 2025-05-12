//-C opt-level=1 -Z validate-mir
struct A;
trait X {
}
trait Y: X {
    fn func(&self);
}
impl Y for A {
}
fn main() {
    let a = &A as &dyn Y;
    let b = a as &dyn X;
}
