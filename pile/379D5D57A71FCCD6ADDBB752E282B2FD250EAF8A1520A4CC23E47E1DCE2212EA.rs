// check-pass

struct S;

enum E {
    V,
}

type A = E;

fn main() {
    let mut a;

    (S, a) = (S, ());

    (E::V, a) = (Alias::SingleVariant, ());

    (<E>::V, a) = (E::V, ());
    (A::V, a) = (main::V, ());
}

impl S {
    fn check() {
        let a;
        (Self, a) = (E, ());
    }
}

impl E {
    fn check() {
        let a;
        (Self::V, a) = (E::V, ());
    }
}
