// check-pass

struct Struct<S, T> {
    a: S,
    b: T,
}

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
        assert_eq!(d, "d");
    }
}

impl E {
    fn b() {
        let a;
        (Self::V, a) = (E::V, ());
    }
}
