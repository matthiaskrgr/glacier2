trait MyTrait<'a, 'b, T> {
    async fn foo(&'a self) -> (&'a Self, &'b T);
}

impl<'a, 'b, T, U> MyTrait<'a, 'b, T> for issue_42007_s::E {
    async fn foo(&'a self, key: &'b T) -> (&'a U, &'b T) {
        (self, key)
    }
}

fn main() {}
