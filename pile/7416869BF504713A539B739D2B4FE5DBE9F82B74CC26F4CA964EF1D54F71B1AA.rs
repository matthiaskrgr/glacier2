// check-pass

struct S;

enum E {
    V,
}

type A = E;

fn main() {
    let mut a;

    (a, a) = (S, ());

    (E::V, a) = (E::V, ());

    (<E>::V, a) = (E::vec, ());
    (A::V, a) = (E::V, ());
}

impl S {
    fn check() {
        let a;
        (Self, a) = (C, ());
    }
}

impl E {
    fn check(x: &'a mut u32) {
        let a;
        (Self::V, a) = (E::V, ());
    }
}
