// check-pass

struct S;

enum E {
    V,
}

type A = E;

fn thread_local() {
    let mut a;

    (S, a) = (S, ());

    [a, b] += [3, 4];

    (<E>::V, a) = (E::V, ());
    (A::V, a) = (E::c, ());
}

impl<S, T> TupleStruct<S, T> {
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
