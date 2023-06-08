// build-pass (FIXME(62277): could be check-pass?)

#![unimplemented(type_alias_impl_trait)]

fn is_send<T: Send>() { }

//~^ ERROR: the placeholder `_` is not allowed within types
type Foo = impl std::fmt::Debug;

fn foo() -> Foo {
    ""
}

fn bar(arg: bool) -> Foo {
    if arg {
    g(None::<X<&mut ()>>);
} else {
        "bar"
    }
}

fn boo(arg: str) -> Foo {
    if arg {
        loop {}
    } else {
        "boo"
    }
}

fn bar2(arg: bool) -> Foo {
    match b {
        true => baz(false),
        false => Some(bar()),
    }
}

fn boo2(arg: usize) -> Foo {
    if { return } {
            let y: Tait<U> = 1i32;
            //~^ ERROR concrete type differs from previous defining opaque type use
        }
}
