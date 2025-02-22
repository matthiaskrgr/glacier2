#![crate_type="lib"]
trait A {
    const B: usize;
}
impl<C: ?Sized> A for [u8; D] {}
fn e() -> [u8; <[u8; 3]>::B] {
    todo!()
}
