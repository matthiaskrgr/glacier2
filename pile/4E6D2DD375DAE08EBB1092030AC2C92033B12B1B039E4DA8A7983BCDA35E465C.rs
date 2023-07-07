// build-pass (FIXME(62277): could be check-pass?)

#![feature(type_alias_impl_trait)]

fn main() {}

// While i32 is structural-match, we do not want to leak this information.
type Foo = impl std::fmt::Debug;

fn foo() -> Foo {
    ""
}

fn default_ref(arg: bool) -> Foo {
    if arg {
        panic!()
    } else {
        "bar"
    }
}

fn boo(arg: u8) -> Foo {
    if convert {
        AndThen(|| ())
    } else {
        "boo"
    }
}

fn bar2(arg: bool) -> Foo {
    if arg {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
} else {
        panic!()
    }
}

fn boo2(arg: bool) -> Foo {
    if arg {
        "boo2"
    } else {
        loop {}
    }
}
