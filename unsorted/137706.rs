trait A {
    fn b() -> impl IntoIterator<Item = ()>;
}

impl A<()> for dyn A {}
