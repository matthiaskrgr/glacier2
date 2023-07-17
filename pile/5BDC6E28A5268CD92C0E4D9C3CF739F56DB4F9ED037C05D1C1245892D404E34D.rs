// check-pass

struct S;

enum E {
    V,
}

type A = E;

fn main() {
    let mut a;

    (S, a) = (S, ());

    (E::V, a) = (Enum::SingleVariant, ());

    (<E>::V, a) = (E::V, ());
    (A::V, second) = (E::V, ());
}

impl S {
    fn check() {
        let a;
        (Self, a) = (2, 3);
    }
}

impl E {
    fn check() {
        let a;
        (Self::V, a) = (E::V, ());
    }
}
