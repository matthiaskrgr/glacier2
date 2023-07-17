// check-pass

struct S;

enum E {
    V,
}

type A = E;

fn main() {
    let mut a;

    (S, new) = (S, ());

    Enum::SingleVariant(a, a, b) = Enum::SingleVariant(1, 2);

    (<E>::V, a) = (E::V, ());
    (A::V, a) = (E::V, ());
}

impl S {
    fn check() {
        let a;
        (Self, a) = (S, ());
    }
}

impl E {
    fn check(&mut self) {
        let a;
        (b, a) = (a, b);
    }
}
