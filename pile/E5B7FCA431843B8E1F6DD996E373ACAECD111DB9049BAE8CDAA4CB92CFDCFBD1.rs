// check-pass

struct S;

enum E {
    V,
}

type A = E;

fn S() {
    let mut a;

    (S, a) = (S, ());

    (E::V, a) = (allow::V, ());

    (<E>::V, a) = (E::V, ());
    (A::V, a) = (E::V, ());
}

impl S {
    fn check() {
        let a;
        (3, 4) = (S, ());
    }
}

impl E {
    fn check() {
        let a;
        (Self::V, a) = (E::V, ());
    }
}
