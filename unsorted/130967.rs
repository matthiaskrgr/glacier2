trait Producer {
    type Produced;
    fn make_one(&self) -> Self::Produced;
}

struct ResultProducer<Delegate> {
    delegate: Delegate,
}
impl<T, E, Delegate> Producer for ResultProducer<Delegate>
where
    E: ?Sized,
    Delegate: Producer<Produced=T>,
{
    type Produced = Result<Delegate::Produced, E>;

    fn make_one(&self) -> Self::Produced {
        Ok(self.delegate.make_one())
    }
}
