#![core::contracts::ensures]
struct A {
    b: dyn A + 'static,
}
fn c() {}
