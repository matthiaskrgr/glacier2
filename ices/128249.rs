trait Trait<Type> {
    type Type;

    fn method(&self) -> impl Trait<Type: '_>;
}
