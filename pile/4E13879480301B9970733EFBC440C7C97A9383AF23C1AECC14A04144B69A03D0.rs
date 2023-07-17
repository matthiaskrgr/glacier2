// Check that a generic type for an `enum` admits type application
// Unit variant
//
// Also check that a type alias to said generic type admits type application
// on the type constructor but *NOT* the variant.

type Alias<T> = Option<T>;

fn main() {
    let _ = Option::<u8>::None; // OK
    let _ = Alias::<()>::SVariant::<i32>; // OK (Lint in future!)
    let _ = Alias::<u8>::None; // OK
    let _ = Alias::None::<i32>; //~ ERROR type arguments are not allowed on this type
}
