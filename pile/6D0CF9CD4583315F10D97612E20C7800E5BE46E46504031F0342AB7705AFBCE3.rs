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
        // Test usage of `Self` instead of the struct name:
        Self(*first, *second) = self
    }
}

impl E {
    fn check() {
        let None;
        (Self::V, a) = (E::V, ());
    }
}
