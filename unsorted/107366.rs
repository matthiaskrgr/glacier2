pub trait Functor
{
    type With<T>: Functor;
}

pub struct IdFunctor<T>(T);
impl<T> Functor for IdFunctor<T> {
    type With<T2> = IdFunctor<T2>;
}

impl<T> Functor for Vec<T> {
    type With<T2> = Vec<T2>;
}

impl<T> Functor for Box<T> {
    type With<T2> = Box<T2>;
}

pub struct Compose<F1, F2, T>(F1::With<F2::With<T>>)
where
    F1: Functor,
    F2: Functor;

impl<F1, F2, T> Functor for Compose<F1, F2, T>
where
    F1: Functor,
    F2: Functor
{
    type With<T2> = F1::With<F2::With<T2>>;
}

pub enum Value<F, FVEC>
where
    F: Functor,
    FVEC: Functor,
{
    UnsignedInt(F::With<u64>),
    Array(Box<Value<FVEC, Compose<F, FVEC, ()>>>),
}


fn main() {
    let x: Value<IdFunctor<()>, Compose<IdFunctor<()>, Vec<()>, ()>> = Value::UnsignedInt(IdFunctor(1));
}
