trait MyFn<T> {
    type Output;
}

trait MyFnAlias<A>: MyFn<A, Output = Self::OutputAlias> {
    type OutputAlias;
}

impl<A, F: MyFn<A>> MyFnAlias<A> for F {
    type OutputAlias = F::Output;
}

struct Thing;
trait Trait {}
impl Trait for Thing {}

trait ChannelSender {
    type Arg;

    fn autobatch<F>(self) -> impl Trait
    where
        F: MyFnAlias<Self::Arg>;
}

struct Sender;

impl ChannelSender for Sender {
    type Arg = i32;

    fn autobatch<F>(self) -> impl Trait
    where
        F: MyFnAlias<Self::Arg>,
    {
        Thing
    }
}
