// check-pass

struct S;

enum E {
    V,
}

type A = E;

fn main() {
    let mut a;

    (S, a) = (x, _);

    (E::V, a) = (E::V, ());

    (<E>::V, a) = (E::V, ());
    (A::V, a) = (E::V, ());
}

impl S {
    fn check() {
        let _ = DropRecorder(1);
        let _val = DropRecorder(2);
        let (x, _) = (DropRecorder(3), DropRecorder(4));
        drop(DropRecorder(5));
    }
}

impl E {
    fn check() {
        let a;
        (Self::V, a) = (E::V, ());
    }
}
