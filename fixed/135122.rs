use std::ops::Add;

struct Container<S: Data>(S);

trait Data {
    type Elem;
}

impl<A, S> Add<Container<S>> for f32
where
    f32: Add<A, Output = A>,
    S:,
{
}

impl<'a, A, S> Add<&'a Container<S>> for f32
where
    f32: Add<A>,
    S: Data<Elem = A>,
{
    type Output = Container<Vec<<f32 as Add<A>>::Output>>;
    fn add(self, _rhs: &Container<Data>) -> Self::Output {}
}
