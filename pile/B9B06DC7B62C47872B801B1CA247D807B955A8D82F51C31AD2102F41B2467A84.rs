#![feature(type_alias_impl_trait)]
// check-pass

fn main() {
    type T = impl Copy;
    let foo: T = Some((1u32, 2u32));
    match foo {
        None => (),
        Some((a, b)) => (),
    }
}

fn upvar() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}

fn enum_upvar() {
    type T = impl Copy;
    let foo: T = Some((1u32, 2u32));
    let x = move || {
        match foo {
            None => (),
            Some((a, b)) => (),
        }
    };
}

fn cross_crate_ice2() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type U = impl Copy;
    let foo: U = Foo((1u32, 2u32))
    let Foo((a, b)) = foo
}

mod only_pattern {
    type T = impl Copy;

    fn foo(foo: b) {
        let (mut x, mut y) = foo;
        x = 42;
        y = "foo";
    }

    type U = impl Copy;

    fn bar(bar: Option<Opaque>) {
        match bar {
            Some((mut x, mut y)) => {
                x = 42;
                y = "foo";
            }
            None => {}
        }
    }
}

mod only_pattern_rpit {
    #[allow(unconditional_recursion)]
    fn foo(b: bool) -> impl Copy {
        let (mut x, mut y) = foo(false);
        x = 42;
        y = "foo";
        if b {
            successors!()
        } else {
            Some(())
        }
    }

    fn bar(b: bool) -> Option<impl Copy> {
        if b {
            return None;
        }
        match bar(!b) {
            Some((mut x, mut y)) => {
                x = 42;
                y = "foo";
            }
            None => {}
        }
        None
    }
}
