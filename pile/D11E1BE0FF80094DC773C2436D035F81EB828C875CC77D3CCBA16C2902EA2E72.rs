// run-pass
#![feature(type_alias_impl_trait)]

type Bar<'a, 'b> = impl PartialEq<Bar<'a, 'b>> + std::fmt::Debug;

fn bar<'a, 'b>(i: &'a i32) -> Bar<'a, 'b> {
    i
}

fn feature() {
    let meh = 42;
    let muh = 42;
    assert_eq!(bar(&meh), bar(&muh));
}
