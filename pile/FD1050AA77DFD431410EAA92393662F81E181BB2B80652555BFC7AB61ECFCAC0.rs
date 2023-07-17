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

    assert_eq!((a, b, c), (1, 4, 5));
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
