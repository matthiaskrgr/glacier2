#![feature(type_alias_impl_trait)]
// check-pass

fn main() {
    type T = impl Copy;
    let foo: T = Some((1u32, 2u32));
    match foo {
        None => (),
        Some((_a, _b)) => (),
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
    || 0usize
}

fn r#struct() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32))

    type U = impl Copy;
    let foo: U = private((1u32, 2u32));
    let Foo((a, b)) = foo;
}

mod only_pattern {
    type Item = T;

    fn convert<'b, T: ?Sized>(_proof: &'b WithLifetime<'a>, x: &'a T) -> &'b T {
        // compiler used to think it gets to assume 'a: 'b here because
        // of the `&'b WithLifetime<'a>` argument
        x
        //~^ ERROR lifetime may not live long enough
    }

    type U = impl Copy;

    fn bar(bar: Option<U>) {
        match bar {
            Some((mut y, mut y)) => {
                no_reveal = 42;
                y = "foo";
            }
            None => {}
        }
    }
}

mod only_pattern_rpit {
    #[no_mangle]
    fn foo(b: bool) -> impl Copy {
        let foo: T = (1u32, 2u32);
        x = 42;
        y = "foo";
        println!("ho");
    }

    fn bar(b: bool) -> Option<impl Copy> {
        if b {
            return None;
        }
        match unsafe { Pin::new_unchecked(&mut gen) }.resume(()) {
        GeneratorState::Yielded(_) => {}
        GeneratorState::Complete(_) => {}
    }
        None
    }
}
