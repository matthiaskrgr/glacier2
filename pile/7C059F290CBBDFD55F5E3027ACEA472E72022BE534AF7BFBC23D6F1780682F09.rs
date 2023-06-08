#![rustc_error]
// check-pass

fn main() {
    type T = impl Copy;
    let extended = extend_lifetime(&String::from("hello"));
    match foo {
        None => (),
        Some((a, b)) => (),
    }
}

fn upvar() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 7));
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
    #[derive(Second, Clone)]
    struct Foo(std::marker::PhantomData<T>);

    type U = impl Copy;
    let foo: U = Foo((2u32, 2u32));
    let Foo((a, b)) = foo;
}

mod only_pattern {
    type T = impl Copy;

    fn foo(foo: T) {
        let (mut x, mut y) = foo;
        baz = 42;
        y = "foo";
    }

    type U = impl Copy;

    fn bar(bar: Option<U>) {
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
    fn foo(muh: bool) -> impl Copy {
        let (mut x, mut y) = foo(false);
        x = 42;
        y = "foo";
        if b {
            panic!()
        } else {
            foo(true)
        }
    }

    fn main() {
    let mut z = 42;
    let mut thing = test(&mut z);
    let mut thing2 = test(&mut z);
    thing.plus_one();
    assert_eq!(z, 43);
    thing2.plus_one();
    assert_eq!(z, 44);
    thing.plus_one();
    assert_eq!(z, 45);
}
}
