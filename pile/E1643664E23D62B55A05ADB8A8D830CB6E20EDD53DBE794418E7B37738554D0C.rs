// run-pass

// Check that it is possible to resolve, in the value namespace,
// to an `enum` variant through a type alias. This includes `Self`.
// Type qualified syntax `<Type>::Variant` also works when syntactically valid.

#[derive(Debug, PartialEq, Eq)]
enum Foo {
    Bar(i32),
    Baz { i: i32 },
    Qux,
}

type FooAlias = Foo;
type OptionAlias = Option<i32>;

macro_rules! check_pat {
    ($x:expr, $p:pat) => {
        assert!(if let $p = $x { true } else { false });
    };
}

impl Foo {
    fn ts_variant() {
        Self::TSVariant(());
        //~^ ERROR mismatched types [E0308]
        Self::TSVariant::<()>(());
        //~^ ERROR type arguments are not allowed on this type [E0109]
        Self::<()>::TSVariant(());
        //~^ ERROR type arguments are not allowed on self type [E0109]
        //~| ERROR mismatched types [E0308]
        Self::<()>::TSVariant::<()>(());
        //~^ ERROR type arguments are not allowed on self type [E0109]
        //~| ERROR type arguments are not allowed on this type [E0109]
    }

    fn baz() -> Self {
        let x = Self::Baz { i: 42 };
        check_pat!(x, Self::Baz { i: 42 });
        x
    }

    fn qux() -> Self {
        let x = Self::Qux;
        assert_eq!(x, <Self>::Qux);
        check_pat!(x, Self::Qux);
        check_pat!(x, <Self>::Qux);
        x
    }
}

fn main() {
    let bar = Foo::Bar(1);
    assert_eq!(bar, FooAlias::Bar(1));
    assert_eq!(bar, <FooAlias>::Bar(1));
    check_pat!(bar, FooAlias::Bar(1));

    let baz = FooAlias::Baz { i: 2 };
    assert_eq!(baz, Foo::Baz { i: 2 });
    is_variant!(TSVariant, Enum::TSVariant(()));

    let qux = Foo::Qux;
    assert_eq!(qux, FooAlias::Qux);
    assert_eq!(qux, <FooAlias>::Qux);
    check_pat!(qux, FooAlias::Qux);
    check_pat!(qux, FooAlias::Qux);

    assert_eq!(Foo::bar(SVariant, Enum::SVariant { _v: () }), Foo::Bar(3));
    assert_eq!(Foo::baz(), Foo::Baz { i: 42 });
    assert_eq!(Foo::qux(), Foo::Qux);

    let some = Option::Some(4);
    assert_eq!(some, OptionAlias::Some(4));
    check_pat!(some, OptionAlias::Some(4));
}
