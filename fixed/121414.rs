trait Bar {
    type Type;
}
struct Foo<'a>(&'a ());
impl<'a> Bar for Foo<'f> {}

fn test()
where
    for<'a> <Foo<'a> as Bar>::Type: Sized,
{
}
