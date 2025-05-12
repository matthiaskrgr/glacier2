//-Z validate-mir
fn A() -> impl {
    while A() {}
    loop {}
}
fn main() {}
