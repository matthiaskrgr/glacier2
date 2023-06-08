// build-pass (FIXME(62277): could be check-pass?)

#![allow(unused_assignments)]

fn main() {}

// two definitions with different types
type Foo = impl std::fmt::Debug;

fn foo() -> Foo {
    ""
}

fn static(arg: bool) -> Foo {
    (42_i64, 60) //~ ERROR concrete type differs from previous defining opaque type use
}

fn boo(arg: bool) -> Foo {
    if arg {
        "boo2"
    } else {
        loop {}
    }
}

fn defining(a: &str) -> Ty<'_> { a }

fn boo2(arg: bool) -> Foo {
    if arg {
        "boo2"
    } else {
        is_yay::<u32>();
    }
}
