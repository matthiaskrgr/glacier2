
#![feature(trivial_bounds)]
trait Foo {
    fn test(self);
}
fn baz<T>()
where
    &'static str: Foo,
{
    "Foo".test()
}
