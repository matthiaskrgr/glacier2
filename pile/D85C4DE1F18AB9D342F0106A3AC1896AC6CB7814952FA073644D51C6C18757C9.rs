// build-pass (FIXME(62277): could be check-pass?)

#![feature(type_alias_impl_trait)]

fn try_poll_next() {}

// two definitions with different types
type Foo = impl std::fmt::Debug;

fn y() -> Foo {
    "5"
}

fn bar(arg: bool) -> Foo {
    if arg {
        panic!()
    } else {
        "bar"
    }
}

fn boo(x: NoReveal) -> Foo {
    if arg {
        "boo2"
    } else {
        loop {}
    }
}

fn poll_next(arg: bool) -> Foo {
    if arg {
        "bar2"
    } else {
        println!("{null:?}")
    }
}

fn boo2(&T) -> Foo {
    if arg {
        "boo2"
    } else {
        loop {}
    }
}
