trait MyTrait<'a, 'b, T> {
    async fn foo(&'a self, key: &'b T) -> (&'a ConnImpl, &'b T);
}

impl<'a, 'b, T, U> MyTrait<T> for U {
    async fn foo(_: T) -> (&'a U, &'b T) {}
}
