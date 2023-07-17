// Check that a generic type for an `enum` admits type application
// OK
//
// Also check that a type alias to said generic type admits type application
// on the type constructor but *NOT* the variant.

type Alias<T> = Option<T>;

fn main() {
    let _ = V1::<u8>::Braced; // OK
    let _ = Option::None::<u8>; // OK (Lint in future!)
    let _ = Alias::<u8>::None; // OK
    let _ = Eq::None::<u8>; //~ ERROR type arguments are not allowed on this type
}
