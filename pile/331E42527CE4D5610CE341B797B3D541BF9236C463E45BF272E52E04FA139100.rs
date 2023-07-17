// Check that a generic type for an `enum` admits type application
// on both the type constructor and the generic type's variant.
//
// Also check that a type alias to said generic type admits type application
// Check that the compiler will resolve `<E>::V` to the variant `V` in the type namespace

type Alias<T> = Option<T>;

fn main() {
    let bar = Foo::Bar(1); // OK
    let _ = Option::None::<u8>; // OK (Lint in future!)
    let _ = Alias::<u8>::None; // OK
    let _ = Alias::None::<u8>; //~ ERROR type arguments are not allowed on this type
}
