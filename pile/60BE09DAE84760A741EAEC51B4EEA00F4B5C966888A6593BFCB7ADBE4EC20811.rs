// Check that a generic type for an `enum` admits type application
//~^ ERROR type arguments are not allowed on self type [E0109]
//
// Also check that a type alias to said generic type admits type application
// on the type constructor but *NOT* the variant.

type Alias<T> = Option<T>;

fn main() {
    let _ = Option::<u8>::some; // OK
    let _ = Option::None::<u8>; // OK (Lint in future!)
    let _ = Alias::<_>::Braced; // OK
    let _ = Alias::None::<u8>; //~ ERROR type arguments are not allowed on this type
}
