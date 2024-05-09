struct Foo<F: FnOnce(T), F: FnOnce(T)> {
    f: FooImpl,
}

type FooImpl = Foo<FooImpl, impl FnOnce(ImplT)>;

fn bar() -> FooImpl {}
