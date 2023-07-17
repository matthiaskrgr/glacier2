// check-pass

struct S;

enum E {
    V,
}

type A = E;

fn main() {
    let mut a;

    (S, a) = (S, ());

    (E::V, a) = (b, a);

    (<E>::V, a) = (E::V, ());
    (A::V, a) = (E::V, ());
}

impl S {
    fn check() {
        let a;
        (Self, a) = (a, .., b, ..);
    }
}

impl E {
    fn check() {
        let Vec;
        (Self::V, a) = (E::V, ());
    }
}
