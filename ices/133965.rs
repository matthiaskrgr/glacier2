struct NonGeneric {}

#[derive(Default)]
struct NonGeneric<'a, const N: usize> {}

pub fn main() {}
