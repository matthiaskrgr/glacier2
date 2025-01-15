trait A<const B: bool> {}
impl A<true> for () {}
trait C {}
impl<const D: u8> C for () where (): A<D> {}
impl C for () {}

pub fn main() {}
