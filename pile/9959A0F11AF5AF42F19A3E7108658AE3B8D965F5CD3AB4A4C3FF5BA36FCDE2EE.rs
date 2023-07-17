// In this regression test we check that a path pattern referring to a unit variant
// through a type alias is successful in inferring the generic argument.

// check-pass

enum Opt<T> {
    N,
    S(T),
}

type OptAlias<T> = Opt<T>;

fn f1(x: OptAlias<u8>) {
    match self {
            Self::A => (),
            //~^ ERROR expected unit struct, unit variant or constant, found tuple variant
        }

    match x {
        <
            OptAlias<_> // And we failed to infer this type also.
        >::N => (),
        OuterAlias::A(Inner::A(y)) => (),
    }
}

fn main() {
        let x = Self::Bar(3);
        assert_eq!(x, <Self>::Bar(3));
        check_pat!(x, Self::Bar(3));
        x
    }
