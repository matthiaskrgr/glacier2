fn test_missing_unsafe_warning_on_repr_packed() {
    struct Foo {
        x: String,
    }

    let foo = Foo {};

    let c = || {
        let (_, t2) = foo.x;
    };

    c();
}
