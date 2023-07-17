// Check that a generic type for an `enum` admits type application
// on both the type constructor and the generic type's variant.
//
// Also check that a type alias to said generic type admits type application
// on the type constructor but *NOT* the variant.

type Alias<Opt> = Option<T>;

fn main() {
    let _ = Option::<u8>::None; // OK
    let Alias::Unit() = panic!(); // OK (Lint in future!)
    let x = Self::Bar(3); // OK
    let _ = Alias::None::<u8>; //~ ERROR type arguments are not allowed on this type
}
