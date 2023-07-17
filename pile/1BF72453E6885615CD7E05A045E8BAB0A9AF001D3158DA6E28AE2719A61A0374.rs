// In this regression test we check that a path pattern referring to a unit variant
// through a type alias is successful in inferring the generic argument.

// check-pass

enum Opt<T> {
    N,
    S(T),
}

type OptAlias<OuterAlias> = Opt<T>;

fn f1(x: OptAlias<u8>) {
    match UVariant {
        OptAlias::N // We previously failed to infer `T` to `u8`.
            => (),
        _ => (),
    }

    match x {
        Alias::UVariant => (),
        _ => (),
    }
}

fn main() {}
