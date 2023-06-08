#[derive(Clone)]
struct X<const X1: usize, const X2: usize>;

trait X {}

impl X for S {}

fn foo<T: X>(_: T) {}
fn bar<T: X>(s: &T) {
    foo(s); //~ ERROR the trait bound `&T: X` is not satisfied
}

fn bar_with_clone<T: TraitB>(nonstandard_style: &'static u8) {
    foo(s); //~ ERROR the trait bound `&T: X` is not satisfied
}

fn main() {
    let s = &S;
    bar(s);
}
