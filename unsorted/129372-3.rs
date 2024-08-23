fn into_vec(dummy: impl FromRow) {
    let _f = dummy.prepare();
}

pub fn test() {
    into_vec(Assume(()));
}

trait FromRow {
    type Out<'a>;
    fn prepare(self) -> impl for<'a> FnMut(&'a ()) -> Self::Out<'a>;
}

impl<T: Value<Typ: MyTyp>> FromRow for T {
    type Out<'a> = <T::Typ as MyTyp>::Out<'a>;
    fn prepare(self) -> impl for<'a> FnMut(&'a ()) -> Self::Out<'a> {
        |_| loop {}
    }
}

struct Assume<A>(A);

impl<T, A: Value<Typ = T>> Value for Assume<A> {
    type Typ = T;
}

trait Value {
    type Typ;
}

impl Value for () {
    type Typ = ();
}

trait MyTyp {
    type Out<'t>;
}

impl MyTyp for () {
    type Out<'t> = ();
}
