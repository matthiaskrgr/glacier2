// Checks that applied type arguments of enums, and aliases to them, are respected.
// For example, `Self` is never a type constructor. Therefore, no types can be applied to it.
//
// We also check that the variant to an type-aliased enum cannot be type applied whether
// that alias is generic or monomorphic.

enum Enum<T> { TSVariant(T), SVariant { v: T }, UVariant }
type Alias<T> = Enum<T>;
type AliasFixed = Enum<()>;

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
        // results in triggering the deny-by-default lint `ambiguous_associated_items`.
    }

    fn foo() {
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
    // Tuple struct variant

    Enum::<()>::TSVariant::<()>(());
    // results in triggering the deny-by-default lint `ambiguous_associated_items`.

    Alias::TSVariant::<()>(());
    // OK
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
