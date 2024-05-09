pub trait Layer<C> {
    type Service;
}

pub trait ParticularService {}

pub trait ParticularServiceLayer<C>: Layer<C> {}

impl<T> ParticularServiceLayer<C> for T
where
    T: Layer<C>,
    T::Service: ParticularService,
{
}

struct ALayer<C>(C);
impl<C> Client<C>
where
    ALayer<C>: ParticularServiceLayer<C>,
{
    fn check(&self) {}
}

struct Client<Service>(C);

impl<C> Client<C> {
    fn check(&self) {}
}
