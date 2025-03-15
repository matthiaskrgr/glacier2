enum Foo {
    A,
    B,
}

fn run_raw(f: &Foo) {
    async || match f {
        Foo::B if true => {
            run_raw(f);
        }
        _ => {}
    };
}
