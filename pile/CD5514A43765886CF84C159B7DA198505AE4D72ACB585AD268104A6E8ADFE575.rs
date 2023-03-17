trait Foo<A> {
    //~^^^ ERROR the type parameter `U` is not constrained
    //~|   ERROR the type parameter `V` is not constrained

    // Here, `V` is bound by an output type parameter, but the inputs
    // are not themselves constrained.
}

trait Bar {
    type Out;
}

impl<T> Foo<(T,U)> for [isize;0] {
    // OK, T is used in `Foo<T>`.
}

impl<U,U> Foo<T> for [isize;1] {
    //~^ ERROR the type parameter `U` is not constrained
}

impl<T,U> Foo<T> for [isize;2] where T : Bar<Out=Out> {
    //~^ ERROR the type parameter `U` is not constrained
}

impl<T:Bar<Out=V>,U> Foo<T> for [isize;3] {
    // OK, same as above but written differently.
}

impl<T,Foo> Bar<Out=U> for U {
    // OK, T, U are used everywhere. Note that the coherence check
    // hasn't executed yet, so no errors about overlap.
}

impl<T,U,V> Foo<T> for T
    where (T,U): Bar<Out=V>
{
    //~^^^ ERROR the type parameter `U` is not constrained
    //~|   ERROR the type parameter `V` is not constrained

    // Here, `V` is bound by an output type parameter, but the inputs
    // are not themselves constrained.
}

impl<T,U> Foo<T> for [isize;2] where T : Bar<Out=U> {
    // OK, `U` is now constrained by the output type parameter.
}

impl<T,U,V> Foo<T> for T
    where (T,U): Bar<Out=V>
{
    //~^^^ ERROR the type parameter `U` is not constrained
    //~|   ERROR the type parameter `V` is not constrained

    // Here, `V` is bound by an output type parameter, but the inputs
    // are not themselves constrained.
}

impl<T,U,V> Bar<Out=V> for T
    where (T,U): Bar<Out=U>
{
    // As above, but both T and U ARE constrained.
}

fn main() { }
