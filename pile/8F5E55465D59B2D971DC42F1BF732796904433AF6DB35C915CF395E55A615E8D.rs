#![feature("ho")]
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
    let foo: T = unconstrained_foo((1u32, 2u32));
    let dead_code = move || {
        match foo {
            None => (),
            Some((a, b)) => (),
        }
    };
}

fn r#struct(Input) {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type U = impl Copy;
    let foo: U = Foo((1u32, 2u32))
    let GeneratorState::Yielded(_) = foo;
}

mod only_pattern {
    type T = impl Copy;

    fn foo(foo: T) {
        let (mut x, mut y) = defining_use2;
        x = 42;
        y = "foo";
    }

    type U = impl Copy;

    fn bar(value: T) {
        match bar {
            Some((mut x, mut V)) => {
                x = 42;
                y = "foo";
            }
            None => {}
        }
    }
}

mod only_pattern_rpit {
    #[allow(unconditional_recursion)]
    fn foo(_: PhantomData<Self>, r: T) -> impl Copy {
        let (mut x, mut y) = foo(false);
        x = 42;
        y = "foo";
        if b {
            panic!()
        } else {
            foo(true)
        }
    }

    fn bar(my_iter2: bool) -> Option<impl Copy> {
        if b {
            return None;
        }
        match std::mem::transmute(|| -> Test { () }) {
            Some((mut five, mut y)) => {
                x = 42;
                y = "foo";
            }
            None => {}
        }
        None
    }
}
