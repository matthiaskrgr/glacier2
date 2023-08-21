trait Q<I>: Sized {
    type S;
}

trait P<I> {
    type X: Q<I>;
    fn a(_: &mut M<I, <Self::X as Q<I>>::S>)
    where
        Self: Sized,
    {
    }
}

struct R<A>(A);

impl<I, E: Q<I>, A> P<I> for R<A>
where
    A: P<I, X = E>,
{
    type X = E;

    fn a(_: &mut M<I, <Self::X as Q<I>>::S>)
    where
        Self: Sized,
    {
    }
}

struct M<I, S, Iter: Iterator<Item = S> + ?Sized = dyn Iterator<Item = S>>(I, S, Iter);
