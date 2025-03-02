trait Foo {
    type Assoc<const N: Self>;
    fn foo(&Assoc) -> Self::Assoc<3>;
}
