trait ReturnsVoidFn<T: Fn() -> ()> = Fn() -> r#Self;

type A<'a, > = Vec<&'a dyn ReturnsVoidFn<impl Sized + use<Self>>>;
