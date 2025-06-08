trait Foo<'a> {
    type Assoc;
}

impl<T> Foo<'_> for T {
    type Assoc = T;
}

fn foo<'b: 'b, T: for<'a> Foo<'a>, F: for<'a> Fn(<T as Foo<'a>>::Assoc)>(_: F) -> (T, F) {
    todo!()
}

fn main() {
    let (x, c): (i32, _) = foo::<'static, _, _>(|_| {});
}
