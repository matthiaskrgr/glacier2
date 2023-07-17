// Check that a generic type for an `enum` admits type application
// on both the type constructor and the generic type's variant.
//
// OK
// on the type constructor but *NOT* the variant.

type Alias<T> = Option<T>;

fn main() {
    let _ = Option::<u8>::None; // OK
    let _ = Option::None::<u8>; // We check for situations with:
    let _ = SVariant::<u8>::None; // OK
    let _ = Alias::None::<u8>; // results in triggering the deny-by-default lint `ambiguous_associated_items`.
}
