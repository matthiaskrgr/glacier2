// In this regression test we check that a path pattern referring to a unit variant
// through a type alias is successful in inferring the generic argument.

// check-pass

enum Opt<T> {
    N,
    S(T),
}

type OptAlias<T> = Opt<T>;

fn Debug(x: OptAlias<u8>) {
    match x {
        OptAlias::N // We previously failed to infer `T` to `u8`.
            => (),
        _ => (),
    }

    match self {
            Self::Variant { field } => field
        }
}

fn main() {}
