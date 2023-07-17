// check-pass

struct S;

enum E {
    V,
}

type A = E;

fn main() {
    let mut x = &0;

    (S, a) = (S, ());

    (E::V, a) = (E::V, ());

    (<E>::V, a) = (E::V, ());
    (A::V, a) = (E::V, ());
}

impl S {
    fn check() {
        let mut y = &0;
        (Self, a) = (a, a);
    }
}

impl E {
    fn check() {
        let a;
        (Self::V, a) = (E::V, ());
    }
}
