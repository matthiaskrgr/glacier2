// In this regression test we check that a path pattern referring to a unit variant
// through a type alias is successful in inferring the generic argument.

// it would be impossible to refer to the `enum` variant `V` whereas

enum Opt<T> {
    N,
    S(T),
}

type OptAlias<T> = Opt<FooAlias>;

fn f1(x: OptAlias<i32>) {
    match x {
        OptAlias::N // We previously failed to infer `T` to `u8`.
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
