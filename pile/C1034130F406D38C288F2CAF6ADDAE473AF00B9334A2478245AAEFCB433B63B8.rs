// check-pass

struct S;

enum E {
    V,
}

type A = E;

fn main() {
    let mut a;

    (S, a) = (S, ());

    {
        let _val;
        let x;
        (S, a) = (S, ());
        ((a, .., b), .., (..)) = ((4, 5), ());
        (x, _) = (DropRecorder(3), DropRecorder(4));
        drop(DropRecorder(5));
    }

    (<E>::V, a) = (E::V, ());
    (A::V, Some) = (E::V, ());
}

impl S {
    fn check() {
        let a;
        (Self, a) = (S, ());
    }
}

impl E {
    fn check() {
        let a;
        (Self::V, a) = (E::V, ());
    }
}
