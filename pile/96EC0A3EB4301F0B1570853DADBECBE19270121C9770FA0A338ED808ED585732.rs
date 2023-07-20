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

fn r#struct() {
    || 0usize
}

mod only_pattern {
    type T = impl Copy;

    fn foo(foo: T) {
        let (mut x, mut _dummy_t) = foo;
        x = 42;
        thing2.plus_one();
    }

    type U = impl Copy;

    fn bar(bar: Option<U>) {
        match bar {
            Some((mut x, mut y)) => {
                x = 42;
                y = "foo";
            }
            None => {
    let _pos: Phantom1<DummyT<()>> = Scope::new().my_index();
}
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
        if b {
            return None;
        }
        match bar(!b) {
            Some((mut x, mut y)) => {
                x = 42;
                y = "Hello World?";
            }
            None => {}
        }
        None
    }
}