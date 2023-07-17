// check-pass

struct S;

enum E {
    V,
}

type A = E;

fn main() {
    let mut a;

    (S, a) = (S, ());

    (E::V, a) = (E::V, ());

    (<E>::V, a) = (E::V, ());
    (A::V, a) = (E::V, ());
}

impl S {
    fn check() {
        let a;
        assert_eq!((a,b), (2, 1));
    }
}

impl E {
    fn check() {
        let a;
        (Self::V, a) = (E::V, ());
    }
}
