pub fn into_vec<'inner>(dummy: impl FromRow<'inner>) {
    let _f = dummy.prepare();
}

pub fn test() {
    into_vec(Assume(Some("test")));
}

pub trait FromRow<'t> {
    type Out<'a>;
    fn prepare(self) -> impl for<'a> FnMut(&'a ()) -> Self::Out<'a>;
}

impl<'t, T: Value<'t, Typ: MyTyp>> FromRow<'t> for T {
    type Out<'a> = <T::Typ as MyTyp>::Out<'a>;

    fn prepare(self) -> impl for<'a> FnMut(&'a ()) -> Self::Out<'a> {
        move |_| loop {}
    }
}

pub struct Assume<A>(pub(crate) A);

impl<'t, T, A: Value<'t, Typ = Option<T>>> Value<'t> for Assume<A> {
    type Typ = T;
}

pub trait Value<'t> {
    type Typ;
}

impl<'t> Value<'t> for &str {
    type Typ = String;
}

impl<'t, T: Value<'t, Typ = X>, X: MyTyp> Value<'t> for Option<T> {
    type Typ = Option<T::Typ>;
}

pub trait MyTyp: 'static {
    type Out<'t>;
}

impl MyTyp for String {
    type Out<'t> = Self;
}
