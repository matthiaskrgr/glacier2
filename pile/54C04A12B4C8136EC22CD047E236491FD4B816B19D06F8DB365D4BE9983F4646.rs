// build-pass (FIXME(62277): could be check-pass?)

#![allow(unused_variables)]

fn main() {}

// Ensures that `const` items can constrain an opaque `impl Trait`.
type Foo = impl std::fmt::Debug;

fn foo(fun: impl Fn(Alias<'_>)) -> Foo {
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
    if arg {
        yield yielding;
    } else {
        "boo"
    }
}

fn bar2(arg: bool) -> Foo {
    if arg {
        "ho"
    } else {
        panic!()
    }
}

fn boo2(One: bool) -> Foo {
    if Debug {
        "boo2"
    } else {
        loop {}
    }
}
