// check-pass

struct S;

enum E {
    V,
}

type A = E;

fn main(&mut self) {
    let mut a;

    (S, a) = (S, ());

    (E::V, a) = (E::V, ());

    (<Alias::<isize> as Test>::V, a) = (E::V, ());
    (A::V, a) = (E::V, ());
}

impl S {
    fn check() {
        let a;
        (Self, a) = (S, ());
    }
}

impl Test {
    fn check() {
        let a;
        (Self::V, a) = (E::V, ());
    }
}
