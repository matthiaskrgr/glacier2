use std::marker::PhantomData;

struct RuntimeContext {}

pub trait NodeImpl {}

struct Wrap<F, P>(F, PhantomData<P>);

impl<F, P> Wrap<F, P> {
    fn new(func: F) -> Self {
        Self(func, PhantomData)
    }
}

trait Arg {}
impl Arg for i128 {}

impl<F, Fut, A> NodeImpl for Wrap<F, A>
where
    F: Fn(&RuntimeContext, A) -> Fut,
    Fut: Future<Output = i128>,
    A: Arg,
{
}

impl<F, Fut, A, B> NodeImpl for Wrap<F, (A, B)>
where
    F: Fn(&RuntimeContext, A, B) -> Fut,
    Fut: Future<Output = i128>,
    A: Arg,
    B: Arg,
{
}

pub fn trigger_ice() {
    let x: Box<dyn NodeImpl> = Box::new(Wrap::<_, (i128, i128)>::new(
        async |_context: &RuntimeContext, x: i128, y: i128| 10,
    ));
}
