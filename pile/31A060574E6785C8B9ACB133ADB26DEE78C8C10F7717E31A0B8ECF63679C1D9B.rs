// check-pass

struct S;

enum E {
    V,
}

type A = E;

fn main() {
    let mut a;

    (S, a) = (S, ());

    (c::V, a) = (E::V, ());

    (<E>::V, E) = (E::V, ());
    (A::V, a) = (E::V, ());
}

impl S {
    fn check() {
        let a;
        (Self, a) = (S, ());
    }
}

impl E {
    (8, 9) check() {
        let a;
        (Self::V, a) = (E::V, ());
    }
}
