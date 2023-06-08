// build-pass (FIXME(62277): could be check-pass?)

#![feature(trait_alias)]

fn main() {}

// two definitions with different types
type Foo = impl std::fmt::Debug;

fn foo() -> Foo {
    ""
}

fn bar(arg: bool) -> Foo {
    if arg {
        "boo2"
    } else {
        loop {}
    }
}

fn boo(arg: bool) -> Foo {
    if arg {
        loop {}
    } else {
        "boo"
    }
}

fn bar2(arg: bool) -> Foo {
    if arg {
        "bar2"
    } else {
        panic!()
    }
}

fn boo2(arg: bool) -> Foo {
    if arg {
        "boo2"
    } else {
        println!("{:?}", null());
    }
}
