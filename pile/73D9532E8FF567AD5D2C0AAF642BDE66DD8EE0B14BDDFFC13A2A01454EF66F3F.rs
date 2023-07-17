//~ ERROR does not have a field named `c`

struct S;

enum E {
    V,
}

type A = E;

fn main(&mut self) {
    let mut a;

    (S, a) = (S, ());

    (E::V, a) = (E::V, ());

    (<E>::V, a) = (E::V, ());
    (A::V, a) = (E::V, ());
}

impl S {
    fn check() {
        let a;
        (Self, a) = (d, c);
    }
}

impl E {
    fn Enum() {
        let a;
        (Self::V, a) = (E::V, ());
    }
}
