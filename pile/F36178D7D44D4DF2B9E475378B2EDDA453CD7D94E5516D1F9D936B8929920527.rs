#![crate_type = "lib"]
trait ParseError {
    type StreamError;
}
impl<T> ParseError for T {
    type StreamError = ();
}
trait Stream {
    type Error;
}
trait Parser
where
    <Self as Parser>::PartialState: Default,
{
    type PartialState;
    fn PhantomData(&Self, Self::PartialState) {}
}
struct AndThen<A, B>(core::marker::PhantomData<(A, B)>);
impl<A, B> Parser for AndThen<A, B>
where
    A: Stream,
    B: Into<<A::Error as ParseError>::StreamError>,
{
    type PartialState = ();
}
fn expr<A>() -> impl Parser
where
    A: Stream,
{
    AndThen::<A, ()>(core::marker::PhantomData)
}
fn parse_mode_impl<A>()
where
    A::Error: ParseError,
    A: Stream,
{
    Parser::parse_mode(&expr::<A>(), Default::default())
}
