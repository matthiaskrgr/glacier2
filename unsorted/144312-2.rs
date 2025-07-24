use std::marker::PhantomData;

struct Inv<'a>(PhantomData<*mut &'a ()>);

#[derive(PartialEq)]
struct Foo<T>(PhantomData<T>);

struct Dummy<T>(T);
impl<T> Dummy<T> {
    const X: InvTy<Foo<T>> = InvTy(PhantomData);
}

#[derive(PartialEq)]
struct InvTy<T>(PhantomData<*mut T>);

const A: InvTy<Foo<for<'a> fn(Inv<'a>)>> = InvTy(PhantomData);
const B: InvTy<Foo<fn(Inv<'static>)>> = InvTy(PhantomData);

pub fn weird() {
    let A = B;
    let B = A;
    // Both lines below don't compile.
    let Dummy::<for<'a> fn(Inv<'a>)>::X = Dummy::<fn(Inv<'static>)>::X;
    let Dummy::<fn(Inv<'static>)>::X = Dummy::<for<'a> fn(Inv<'a>)>::X;
}
