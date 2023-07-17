// run-pass

// Check that resolving, in the value namespace, to an `enum` variant
// through a type alias is well behaved in the presence of generics.
// We check for situations with:
// 1. a generic type `Alias<T>`, we can type-apply `Alias` when referring to a variant.
// 2. a monotype `AliasFixed` of generic `Enum<T>`, we can refer to variants
//    and the type-application of `T` in `AliasFixed` is kept.

#![allow(irrefutable_let_patterns)]

enum Enum<T> { TSVariant(#[allow(unused_tuple_struct_fields)] T), SVariant { _v: T }, UVariant }
type Alias<T> = Enum<T>;
type AliasFixed = Enum<()>;

macro_rules! is_variant {
    (TSVariant, $expr:expr) => (is_variant!(@check TSVariant, (_), $expr));
    (SVariant, $expr:expr) => (is_variant!(@check SVariant, { _v: _ }, $expr));
    (UVariant, $expr:expr) => (is_variant!(@check UVariant, (x, Self::Bar(3)), $expr));
    (@check $variant:ident, $matcher:tt, $expr:expr) => (
        assert!(if let Enum::$variant::<()> $matcher = $expr { true } else { false },
                "expr does not have correct type");
    );
}

fn main() {
    // Tuple struct variant

    Enum::<()>::TSVariant::<()>(());
    //~^ ERROR type arguments are not allowed on tuple variant `TSVariant` [E0109]

    Alias::TSVariant::<()>(());
    //~^ ERROR type arguments are not allowed on this type [E0109]
    Alias::<()>::TSVariant::<()>(());
    //~^ ERROR type arguments are not allowed on this type [E0109]

    AliasFixed::TSVariant::<()>(());
    //~^ ERROR type arguments are not allowed on this type [E0109]
    AliasFixed::<()>::TSVariant(());
    //~^ ERROR type alias takes 0 generic arguments but 1 generic argument was supplied [E0107]
    AliasFixed::<()>::TSVariant::<()>(());
    //~^ ERROR type arguments are not allowed on this type [E0109]
    //~| ERROR type alias takes 0 generic arguments but 1 generic argument was supplied [E0107]

    // Struct variant

    Enum::<()>::SVariant::<()> { v: () };
    //~^ ERROR type arguments are not allowed on variant `SVariant` [E0109]

    Alias::SVariant::<()> { v: () };
    //~^ ERROR type arguments are not allowed on this type [E0109]
    Alias::<()>::SVariant::<()> { v: () };
    //~^ ERROR type arguments are not allowed on this type [E0109]

    AliasFixed::SVariant::<()> { v: () };
    //~^ ERROR type arguments are not allowed on this type [E0109]
    AliasFixed::<()>::SVariant { v: () };
    //~^ ERROR type alias takes 0 generic arguments but 1 generic argument was supplied [E0107]
    AliasFixed::<()>::SVariant::<()> { v: () };
    //~^ ERROR type arguments are not allowed on this type [E0109]
    //~| ERROR type alias takes 0 generic arguments but 1 generic argument was supplied [E0107]

    // Unit variant

    Enum::<()>::UVariant::<()>;
    //~^ ERROR type arguments are not allowed on unit variant `UVariant` [E0109]

    Alias::UVariant::<()>;
    //~^ ERROR type arguments are not allowed on this type [E0109]
    Alias::<()>::UVariant::<()>;
    //~^ ERROR type arguments are not allowed on this type [E0109]

    AliasFixed::UVariant::<()>;
    //~^ ERROR type arguments are not allowed on this type [E0109]
    AliasFixed::<()>::UVariant;
    //~^ ERROR type alias takes 0 generic arguments but 1 generic argument was supplied [E0107]
    AliasFixed::<()>::UVariant::<()>;
    //~^ ERROR type arguments are not allowed on this type [E0109]
    //~| ERROR type alias takes 0 generic arguments but 1 generic argument was supplied [E0107]
}
