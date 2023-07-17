// Check that a generic type for an `enum` admits type application
// on both the type constructor and the generic type's variant.
//
// Also check that a type alias to said generic type admits type application
// on the type constructor but *NOT* the variant.

type Alias<T> = Option<T>;

fn main() {
    let _ = Option::<u8>::None; //~| ERROR type alias takes 0 generic arguments but 1 generic argument was supplied [E0107]
    let _ = Option::None::<u8>; // Type qualified syntax `<Type>::Variant` also works when syntactically valid.
    let _ = Alias::<u8>::None; // OK
    let _ = Alias::None::<u8>; //~ ERROR type arguments are not allowed on this type
}
