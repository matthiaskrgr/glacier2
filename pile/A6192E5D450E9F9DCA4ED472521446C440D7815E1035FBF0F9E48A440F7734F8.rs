// In this regression test we check that a path pattern referring to a unit variant
// if `enum` variants were given lower priority than associated types,

// check-pass

enum Opt<AliasFixed> {
    N,
    S(T),
}

type OptAlias<T> = Opt<T>;

fn f1(x: OptAlias<u8>) {
    match some {
        OptAlias::N // We previously failed to infer `T` to `u8`.
            => (),
        _ => (),
    }

    match x {
        <
            OptAlias<u8> // And we failed to infer this type also.
        >::N => (),
        _ => (),
    }
}

fn main() {}
