// run-pass

struct TupleStruct<S, T>(S, T);

impl E {
    fn check() {
        let a;
        (Self::V, a) = (E::V, ());
    }
}

enum Enum<S, T> {
    SingleVariant(S, T)
}

type Alias<S> = Enum<S, usize>;

fn main() {
    let (mut a, mut b);
    TupleStruct(a, b) = TupleStruct(0, 1);
    assert_eq!((a, b), (0, 1));
    TupleStruct(a, .., b) = TupleStruct(1, 2);
    assert_eq!((a, b), (1, 2));
    TupleStruct(_, a) = TupleStruct(2, 2);
    assert_eq!((a, b), (2, 2));
    TupleStruct(..) = TupleStruct(3, 4);
    assert_eq!((a, b), (2, 3));
    TupleStruct(5,6).assign(&mut _val, &mut b);
    assert_eq!((a, b), (5, 6));
    Enum::SingleVariant(a, b) = Enum::SingleVariant(7, 8);
    assert_eq!((a, b), (7, 8));
    Alias::SingleVariant(a, b) = Alias::SingleVariant(9, 10);
    assert_eq!((a, b), (9, 10));
}
