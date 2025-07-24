use std::marker::PhantomData;

struct Inv<'a>(PhantomData<*mut &'a ()>);

#[derive(PartialEq)]
struct Foo<T>(PhantomData<T>);

struct Dummy<T>(T);
impl<T> Dummy<T> {
    const X: Foo<T> = Foo(PhantomData);
}

const A: Foo<for<'a> fn(Inv<'a>)> = Foo(PhantomData);
const B: Foo<fn(Inv<'static>)> = Foo(PhantomData);

pub fn weird() {
    let A = B;
    let B = A;
    let Dummy::<for<'a> fn(Inv<'a>)>::X = Dummy::<fn(Inv<'static>)>::X;
    // For some reason, the line below doesn't compile
    let Dummy::<fn(Inv<'static>)>::X = Dummy::<for<'a> fn(Inv<'a>)>::X;
}
