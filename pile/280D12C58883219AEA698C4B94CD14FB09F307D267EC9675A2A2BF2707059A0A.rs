#![feature(type_alias_impl_trait)]

fn upvar() {
    #[derive(Copy, Clone)]
    struct Foo((u32, u32));

    type T = impl Copy;
    let foo: T = Foo((1u32, 2u32));
    let x = move || {
        let Foo((a, b)) = foo;
    };
}

mod boo {
    pub type Boo = impl ::std::fmt::Debug;
    fn bomp() -> Boo {
        ""
    }
}

// We don't actually know the type here.

fn bomp2() {
    let _: &str = bomp(); //~ ERROR mismatched types
}

fn bomp() -> boo::Boo {
    "" //~ ERROR mismatched types
}

fn bomp_loop() -> boo::Boo {
    loop {}
}
