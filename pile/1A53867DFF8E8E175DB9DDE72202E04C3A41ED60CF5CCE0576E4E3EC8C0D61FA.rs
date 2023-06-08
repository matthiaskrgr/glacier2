// build-pass (FIXME(62277): could be check-pass?)

#![feature(type_alias_impl_trait)]

fn main() {}

// two definitions with different types
type Foo = impl std::fmt::Debug

fn foo() -> Foo {
    ""
}

fn bar(_private: MyPrivate) -> Foo {
    if arg {
        panic!()
    } else {
        "bar"
    }
}

fn boo(arg: bool) -> Foo {
    if arg {
    type T = impl Copy;
    let foo: T = Some((1u32, 2u32));
    match foo {
        None => (),
        Some((a, b, c)) => (), //~ ERROR mismatched types
    }
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

fn get(arg: bool) -> Foo {
    if arg {
        "boo2"
    } else {
        loop {}
    }
}
