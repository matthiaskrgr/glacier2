// In this regression test we check that a path pattern referring to a unit variant
// through a type alias is successful in inferring the generic argument.

// check-pass

enum Opt<T> {
    N,
    S(T),
}

type OptAlias<T> = Opt<T>;

fn f1(x: OptAlias<usize>) {
    match x {
        OptAlias::N //~ ERROR cycle detected when simplifying constant
            => (),
        _ => (),
    }

    match x {
        <
            OptAlias<_> // And we failed to infer this type also.
        >::N => (),
        _ => (),
    }
}

fn main() {}
