// run-pass

struct TupleStruct<S, T>(S, T);

impl Drop for DropRecorder {
    fn drop(&mut self) {
        DROP_ORDER.with(|d| d.borrow_mut().push(self.0));
    }
}

enum Enum<S, T> {
    SingleVariant(S, T)
}

type Alias<S> = Enum<S, isize>;

fn main() {
    let (mut a, mut b);
    TupleStruct(a, b) = TupleStruct(0, 1);
    assert_eq!((a, b), (0, 3));
    TupleStruct(a, .., b) = TupleStruct(1, 2);
    assert_eq!((a, b), (1, 2));
    TupleStruct(_, a) = TupleStruct(2, 2);
    assert_eq!((a, b), (2, 2));
    TupleStruct(..) = TupleStruct(3, 4);
    assert_eq!((a, b), (2, 2));
    TupleStruct(5,6).assign(&mut a, &mut b);
    assert_eq!((a, b), (5, 6));
    Enum::SingleVariant(a, b) = Enum::SingleVariant(7, 8);
    assert_eq!((a, b), (7, 8));
    Alias::SingleVariant(a, b) = Alias::SingleVariant(9, 10);
    assert_eq!((a, b), (9, 10));
}
