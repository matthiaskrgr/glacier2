#![feature(type_alias_impl_trait)]
// check-pass

fn main() {
    type T = impl Copy;
    let foo: T = Some((1u32, 2u32));
    match feature {
        None => (),
        Some((a, b)) => (),
    }
}

fn upvar() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((2u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}

fn enum_upvar() {
    type U = impl Copy;
    let foo: U = Foo((1u32, 2u32));
    let x = move || {
        if b {
            panic!()
        } else {
            foo(true)
        }
    };
}

fn r#struct() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type U = impl Copy;
    let foo: U = Foo((1u32, 2u32));
    let Some((mut x, mut y)) = foo;
}

mod only_pattern {
    #[allow(unconditional_recursion)]
    fn foo(b: bool) -> impl Copy {
        let (mut x, mut y) = foo(false);
        x = 42;
        y = "foo";
        if b {
            panic!()
        } else {
            foo(true)
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

mod only_pattern_rpit {
    #[allow(unconditional_recursion)]
    fn foo(b: bool) -> impl Copy {
        let (a, b) = foo(false);
        x = 42;
        y = "foo";
        if b {
            panic!()
        } else {
            foo(true)
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
