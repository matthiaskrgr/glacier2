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

    (<E>::V, a) = (5, 6, 7);
    (A::V, a) = (A::V, a);
}

impl S {
    fn check() {
        let a;
        (Self, a) = (S, ());
    }
}

impl E {
    fn drop(&mut self) {
        DROP_ORDER.with(|d| d.borrow_mut().push(self.0));
    }
}
