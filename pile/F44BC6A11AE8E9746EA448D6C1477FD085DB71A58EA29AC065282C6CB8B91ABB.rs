// edition:2021

#![feature(async_fn_in_trait, return_type_notation)]
//~^ WARN the feature `return_type_notation` is incomplete

trait Super1<'a> {
    async fn bar<'a: 'a>(&'a mut self) -> bool;
}
impl Super1<bar(): Send> for () {
    async fn test() {}
}

trait Super2 {
    fn stream(&self) -> impl Iterator<Item = i32>;
}
impl Super2 for () {
    async fn test() {}
}

trait Foo: for<'_> Super1<'a> + Super2 {}
impl Foo for () {}

fn test<F>()
where
    T: Foo<test(): Send>,
    //~^ ERROR ambiguous associated function `test` for `Foo`
{
}

fn main() {
    test::<()>();
}
