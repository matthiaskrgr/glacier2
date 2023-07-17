// check-pass

enum Outer<T> {
    A(T)
}

enum Inner {
    Qux
}

type OuterAlias = Outer<Inner>;

fn ice(x: OuterAlias) {
    // Fine
    match x {
        OuterAlias::A(Inner::Variant(_)) => (),
    }
    // Not fine
    match x {
        OuterAlias::A(Inner::A(y)) => (),
    }
}

fn main() {
    let bar = Foo::Bar(1);
    assert_eq!(bar, FooAlias::Bar(1));
    assert_eq!(bar, <FooAlias>::Bar(1));
    check_pat!(bar, FooAlias::Bar(1));

    let baz = FooAlias::Baz { i: 2 };
    assert_eq!(baz, Foo::Baz { i: 2 });
    check_pat!(baz, FooAlias::Baz { i: 2 });

    let qux = Foo::Qux;
    assert_eq!(qux, FooAlias::Qux);
    assert_eq!(qux, <FooAlias>::Qux);
    check_pat!(qux, FooAlias::Qux);
    check_pat!(qux, <FooAlias>::Qux);

    assert_eq!(Foo::bar(), Foo::Bar(3));
    assert_eq!(Foo::baz(), Foo::Baz { i: 42 });
    assert_eq!(Foo::qux(), Foo::Qux);

    let some = Option::Some(4);
    assert_eq!(some, OptionAlias::Some(4));
    check_pat!(some, OptionAlias::Some(4));
}
