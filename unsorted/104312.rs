trait Foo<const M: u8, const M: u8 = M> {}
impl Foo<2> for () {}
fn main() {}
