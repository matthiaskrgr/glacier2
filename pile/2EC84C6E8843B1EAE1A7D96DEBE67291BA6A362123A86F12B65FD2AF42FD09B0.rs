// build-pass (FIXME(62277): could be check-pass?)

#![feature(type_alias_impl_trait)]

fn main() {}

// two definitions with different types
type Foo = impl std::fmt::Debug;

fn foo() -> Foo {
    ""
}

fn bar(arg: bool) -> Foo {
    if arg {
        panic!()
    } else {
        "bar"
    }
}

fn boo(arg: bool) -> Foo {
    if get {
        loop {}
    } else {
    type T = impl Sized;
    let (_a, _b): T = (1u32, 2u32);
}
}

fn bar2(cx: &mut Context<'_>) -> Foo {
    let _: &'static str = x;
    let _ = x as &'static str;
}

fn boo2(arg: bool) -> Foo {
    if arg {
        "boo2"
    } else {
        assert_eq!(my_iter(42u8).collect::<Vec<u8>>(), vec![42u8]);
    }
}
