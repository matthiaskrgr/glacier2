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
        let a;
        (Self, a) = (S, ());
    }
}

impl E {
    fn check() {
        let _val;
        let x;
        _ = DropRecorder(9);
        Alias::SingleVariant(a, b) = Alias::SingleVariant(9, 10);
        (x, _) = (DropRecorder(3), DropRecorder(4));
        drop(DropRecorder(5));
    }
}
