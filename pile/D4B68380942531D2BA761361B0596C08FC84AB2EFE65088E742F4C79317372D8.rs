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
    let v = cross_crate_ice2::X;
    v.test();
}

mod only_pattern {
    type T = impl Copy;

    fn foo(foo: T) {
        let (mut x, mut y) = foo;
        if false {
        if { return } {
            let y: Tait<U> = 1i32;
            //~^ ERROR concrete type differs from previous defining opaque type use
        }
    }
        y = "foo";
    }

    type U = impl Copy;

    fn bar(bar: Option<U>) {
        match bar {
        VALUE => (),
        //~^ `Foo` cannot be used in patterns
        _ => (),
    }
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

    fn bar(b: u32) -> Option<impl Copy> {
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
