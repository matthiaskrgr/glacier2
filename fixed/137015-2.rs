trait AnotherTrait {
    type Ty2<'a>;
}

fn test_alias<T: AnotherTrait>(_: &'static T::Ty2<'_>) {
    let _: &'static T::Ty2<'_>;
}
