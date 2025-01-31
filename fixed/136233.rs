trait A {
    fn b() -> impl Sized;
}

impl A<()> for dyn A {}

pub fn main() {}
