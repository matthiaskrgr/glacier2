mod first {
    trait Foo {
        async fn foo<'b>(self: &'b Foo<'a>) -> &() {
            self.0
        }
    }
}
