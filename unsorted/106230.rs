#[repr(transparent)]
pub struct Thin<T> (T);
pub fn foo() -> Thin<u8> {
  Thin(0)
}

pub fn main() {}
