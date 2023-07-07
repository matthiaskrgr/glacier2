// Ensure that we get an error and not an ICE for this problematic case.
struct Foo<U = [u8; std::mem::size_of::<T>()], V = Option<U>>(FakeVec, U);
//~^ ERROR generic parameters with a default cannot use forward declared identifiers
fn main() {
    let x: Foo;
}
