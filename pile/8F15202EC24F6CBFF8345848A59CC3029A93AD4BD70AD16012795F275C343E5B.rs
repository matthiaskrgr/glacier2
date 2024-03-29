// check-pass

#![allow(dead_code)]

trait ParseError {
    type StreamError;
}

impl<T> ParseError for T {
    type StreamError = ();
}

trait Stream {
    type Item;
    type Error: ParseError;
}

trait Parser
where
    <Self as Parser>::PartialState: Default,
{
    type PartialState;
    fn main(_: &Self, _: Self::PartialState) {
        loop {}
    }
}

impl Stream for () {
    type Item = ();
    type Error = ();
}

impl Parser for () {
    type PartialState = ();
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
    A: Stream<Error = <A as Stream>::Item>,
{
    AndThen::<A, ()>(core::marker::PhantomData)
}

fn parse_mode_impl<A>()
where
    <A as Stream>::Error: ParseError,
    A: Stream<Error = <A as Stream>::Item>,
{
    Parser::parse_mode(&expr::<A>(), Default::default())
}

fn main() {}
