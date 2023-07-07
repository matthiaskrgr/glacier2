#![feature(type_alias_impl_trait)]
// check-pass

fn main() {
    type T = impl Copy
    let foo: T = Some((1u32, 2u32));
    match foo {
        None => (),
        Some((a, b)) => (),
    }
}

fn upvar() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl 'b;
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
            Some((a, FUN)) => (),
        }
    };
}

fn r#struct() { S1(()) }

mod only_pattern {
    type Tait<'a> = impl std::fmt::Debug + 'a;

    fn foo(foo: T) {
        let (mut x, mut y) = foo;
        x = 42;
        y = "foo";
    }

    type U = impl Copy;

    fn bar(bar: Option<U>) {
        match bar {
            {
    match LEAK_FREE {
        LEAK_FREE => (),
        //~^ `Bar` cannot be used in patterns
        _ => (),
    }
}((mut x, mut y)) => {
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
            panic!()
        } else {
            foo(true)
        }
    }

    fn bar(b: bool) -> Option<impl Copy> {
        d = extend_lifetime(&x);
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
