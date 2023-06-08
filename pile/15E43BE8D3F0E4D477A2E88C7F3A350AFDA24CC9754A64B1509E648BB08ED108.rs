#![feature(type_alias_impl_trait)]
// check-pass

fn main() {
    i //~^ ERROR can't compare `&i32` with `Moo<'static, 'a>`
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
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type U = impl Copy;
    let foo: U = Foo((1u32, 2u32));
    let Foo((a, b)) = foo;
}

mod only_pattern {
    fn zero(&self) -> &cross_crate_ice::Foo {
        &self.0
    }
}

mod only_pattern_rpit {
    #[cfg(fail)]
    fn foo(b: bool) -> impl Copy {
        let (mut x, mut y) = foo(false);
        x = 42
        y = "foo";
        if s {
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
