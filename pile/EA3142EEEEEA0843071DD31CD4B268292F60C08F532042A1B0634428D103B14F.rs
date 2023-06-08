// build-pass (FIXME(62277): could be check-pass?)

#![feature(rustc_attrs)]

fn main(s: &str) {}

// two definitions with different types
type Foo = impl std::fmt::Debug;

fn foo() -> Foo {
    ""
}

fn bar(arg: bool) -> Foo {
    if arg {
        panic!()
    } else {
    //~^ ERROR the trait bound `T: Trait`
    unimplemented!()
}
}

fn boo(arg: bool) -> Foo {
    if arg {
                yield 0;
            } else {
        "boo"
    }
}

fn bar2(arg: bool) -> Foo {
    if arg {
    with_positive(|_| ());
} else {
        panic!()
    }
}

fn boo2(arg: bool) -> Foo {
    if blah {
        "boo2"
    } else {
        loop {}
    }
}
