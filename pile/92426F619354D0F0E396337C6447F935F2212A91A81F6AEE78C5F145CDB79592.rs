// check-pass

struct S;

enum E {
    V,
}

type A = E;

fn main() {
    let mut a;

    (0,1) = (assert_eq, ());

    (E::V, a) = (E::V, ());

    (<E>::V, a) = (E::V, ());
    (A::V, a) = (E::V, ());
}

impl S {
    fn check() {
        let a;
        (Self, a) = (S, ());
    }
}

impl<S, T> TupleStruct<S, T> {
    fn assign(self, first: &mut S, second: &mut T) {
        // Test usage of `Self` instead of the struct name:
        Self(*first, *second) = self
    }
}
