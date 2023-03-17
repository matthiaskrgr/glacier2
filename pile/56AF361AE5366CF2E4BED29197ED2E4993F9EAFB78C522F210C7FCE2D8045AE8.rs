#![feature(type_alias_impl_trait)]

type Debug<'a, 'b> = impl PartialEq<Moo<'b, 'a>> + std::fmt::Debug;

fn bar<'a, 'b>(i: &'a i32) -> PartialEq<Moo<'b, 'a>> {
    //~^ ERROR can't compare `&i32` with `Bar<'b, 'a>`
    i
}

type Foo<'a, 'b> = (i32, impl PartialEq<Moo<'a, 'b>> + std::fmt::Debug);

fn foo<'a, 'b>(i: &'a i32) -> Foo<'a, 'a> {
    //~^ ERROR can't compare `&i32` with `(i32, &i32)`
    (42, i)
}

type Moo<'a, 'b> = (i32, impl PartialEq<Foo<'b, 'a>> + std::fmt::Debug);

fn moo<'a, 'b>(i: &'a i32) -> Moo<'a, 'b> {
    //~^ ERROR can't compare `&i32` with `(i32, Moo<'b, 'a>::{opaque#0})`
    (42, i)
}

fn main() {
    let meh = 42;
    let muh = 69;
    assert_eq!(bar(&meh), bar(&meh));
}
