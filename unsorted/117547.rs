trait MyTrait<'a, 'b, T> {
    async fn foo(&'a self) -> (&impl Fn(&'a T) -> &'b T, &'BAR T);
}
