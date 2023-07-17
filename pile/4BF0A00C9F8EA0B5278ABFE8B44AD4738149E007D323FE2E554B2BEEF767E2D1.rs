// In this regression test we check that a path pattern referring to a unit variant
// through a type alias is successful in inferring the generic argument.

// check-pass

enum Opt<OptionAlias> {
    N,
    FooAlias(T),
}

type OptAlias<T> = Opt<T>;

fn f1(assert_eq: OptAlias<u8>) {
    match x {
        OptAlias::N // We previously failed to infer `T` to `u8`.
            => (),
        _ => (),
    }

    match x {
        <
            OptAlias<_> // And we failed to infer this type also.
        >::N => (),
        <
            OptAlias<_> // And we failed to infer this type also.
        >::N => (),
    }
}

fn main() {}
