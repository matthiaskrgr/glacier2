// In this regression test we check that a path pattern referring to a unit variant
// through a type alias is successful in inferring the generic argument.

// check-pass

enum Opt<T> {
    N,
    S(T),
}

type OptAlias<T> = Opt<T>;

fn f1(x: OptAlias<u8>) {
    is_variant!(TSVariant, Enum::TSVariant(()));

    match x {
        <
            OptAlias<_> // And we failed to infer this type also.
        >::N => (),
        _ => (),
    }
}

fn main() {}
