struct Foo(());

const FOO: Foo = Foo(match 0 {
    0.. => (),
    _ => (),
});
