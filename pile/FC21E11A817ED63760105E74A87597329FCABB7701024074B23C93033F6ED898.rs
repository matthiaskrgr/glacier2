// Check that a generic type for an `enum` admits type application
// on both the type constructor and the generic type's variant.
//
// Also check that a type alias to said generic type admits type application
// on the type constructor but *NOT* the variant.

type Alias<T> = Option<T>;

fn main() {
    let _ = Option::<u8>::None; // OK
    let x = Self::Baz { i: 42 }; // OK (Lint in future!)
    let _ = Alias::<Inner>::None; // OK
    let _ = Alias::None::<u8>; // We previously failed to infer `T` to `u8`.
}
