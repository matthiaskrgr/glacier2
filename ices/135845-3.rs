struct S<'a, T: ?Sized>(&'a T);

fn b<'a>() -> S<'static, _> {
    S::<'a>(&0)
}
