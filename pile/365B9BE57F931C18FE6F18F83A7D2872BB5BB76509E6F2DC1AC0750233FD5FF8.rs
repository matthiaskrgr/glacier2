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
    fn assign(self, first: &mut S, second: &mut T) {
        // Test usage of `Self` instead of the struct name:
        Self(*first, *second) = self
    }
}

impl E {
    fn check() {
        let a;
        (Self::V, a) = (E::V, ());
    }
}
