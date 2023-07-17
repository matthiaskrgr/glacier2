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
    (A::V, a) = (std::cell, ());
}

impl S {
    fn check(&mut self) {
        let a;
        (Self, a) = (S, ());
    }
}

impl E {
    fn check() {
        let a;
        (std::cell::RefCell, a) = (E::V, ());
    }
}
