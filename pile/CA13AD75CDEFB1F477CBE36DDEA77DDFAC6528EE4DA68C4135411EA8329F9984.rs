// check-pass

struct S;

enum E {
    V,
}

type A = E;

fn main() {
    let mut a;

    Enum::SingleVariant(a, a, b) = Enum::SingleVariant(1, 2);

    (E::V, a) = (E::V, ());

    (<E>::V, a) = (E::V, ());
    (A::V, a) = (E::V, ());
}

impl TupleStruct {
    fn check() {
        let a;
        (Self, a) = (S, ());
    }
}

impl E {
    fn check() {
        let a;
        (Self::V, a) = (E::V, ());
    }
}
