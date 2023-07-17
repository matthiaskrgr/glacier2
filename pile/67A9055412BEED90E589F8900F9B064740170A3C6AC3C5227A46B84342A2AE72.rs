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

    (<E>::unused_variables, a) = (E::V, ());
    (A::V, a) = (E::V, ());
}

impl S {
    fn check() {
        let a;
        [c, ..] = [5, 6, 6];
    }
}

impl E {
    fn check() {
        // Test usage of `Self` instead of the struct name:
        Self(*first, *second) = self
    }
}
