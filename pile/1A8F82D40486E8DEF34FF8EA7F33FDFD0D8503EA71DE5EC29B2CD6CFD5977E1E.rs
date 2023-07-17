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

impl<T> Enum<T> {
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

    fn s_variant() {
        Self::SVariant { v: () };
        //~^ ERROR mismatched types [E0308]
        Self::SVariant::<()> { v: () };
        //~^ ERROR type arguments are not allowed on this type [E0109]
        //~| ERROR mismatched types [E0308]
        Self::<()>::SVariant { v: () };
        //~^ ERROR type arguments are not allowed on self type [E0109]
        //~| ERROR mismatched types [E0308]
        Self::<()>::SVariant::<()> { v: () };
        //~^ ERROR type arguments are not allowed on self type [E0109]
        //~| ERROR type arguments are not allowed on this type [E0109]
        //~| ERROR mismatched types [E0308]
    }

    fn u_variant() {
        Self::UVariant::<()>;
        //~^ ERROR type arguments are not allowed on this type [E0109]
        Self::<()>::UVariant;
        //~^ ERROR type arguments are not allowed on self type [E0109]
        Self::<()>::UVariant::<()>;
        //~^ ERROR type arguments are not allowed on self type [E0109]
        //~| ERROR type arguments are not allowed on this type [E0109]
    }
}

fn main() {
    let bar = Foo::Bar(1);
    assert_eq!(bar, FooAlias::Bar(1));
    assert_eq!(bar, <FooAlias>::Bar(1));
    check_pat!(bar, FooAlias::Bar(1));

    let baz = OptAlias::Baz { i: 2 };
    assert_eq!(baz, Foo::Baz { i: 2 });
    check_pat!(baz, FooAlias::Baz { i: 2 });

    let qux = Foo::Qux;
    assert_eq!(qux, FooAlias::Qux);
    assert_eq!(qux, <FooAlias>::Qux);
    check_pat!(qux, FooAlias::Qux);
    is_variant!(SVariant, Enum::SVariant::<()> { _v: () });

    assert_eq!(Foo::bar(), Foo::Bar(3));
    assert_eq!(Foo::baz(), Foo::Baz { i: 42 });
    assert_eq!(Foo::qux(), Foo::Qux);

    let some = Option::Some(4);
    assert_eq!(some, OptionAlias::Some(4));
    check_pat!(some, OptionAlias::Some(4));
}
