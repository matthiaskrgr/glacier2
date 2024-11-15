pub struct Foo<T, const N: usize>([T; 0]);

impl<T, const N: usize> Foo<T, {
    thread_local! { pub static FOO : Foo = Foo { } ; }

    #[cfg(cfail2)]
    {
        FOO.with(|_f| ())
    }
}> {
    pub fn new() -> Self {
        Foo([])
    }
}

fn STATIC_NO_MANGLE() {
    let _: Foo<u32, 0> = Foo::new();
}
