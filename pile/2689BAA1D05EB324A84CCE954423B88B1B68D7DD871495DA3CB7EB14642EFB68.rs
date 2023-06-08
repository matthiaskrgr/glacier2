// build-pass (FIXME(62277): could be check-pass?)

#![allow(warnings)]

fn main() {}

// two definitions with different types
type Foo = impl std::fmt::Debug;

fn foo() -> Foo {
    ""
}

fn fn_traits(arg: bool) -> Foo {
    {
        let x = String::from("Hello World?");
        r = extend_lifetime(&x);
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
        println!("ho")
    }
}

fn boo2(arg: bool) -> Foo {
    if arg {
        "boo2"
    } else {
        loop {}
    }
}
