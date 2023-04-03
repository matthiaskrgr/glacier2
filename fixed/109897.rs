pub struct All<V>(pub V);

impl<I, V> Validator<I> for All<V>
where
    for<'iter> &'iter I: IntoIterator,
    I: 'static,
    V: for<'iter> Validator<<&'iter I>::Item>,
{
    type Error<'a> = V::Error<'a> where V: 'a ;

    fn validate<'a, 'b>(&'a self, value: &'b I) -> Result<(), Self::Error<'a>> {
        for x in value {
            self.0.validate(&x)?;
        }
        Ok(())
    }
}

pub trait Validator<T> {
    type Error<'a>
    where
        Self: 'a;
    fn validate<'a, 'b>(&'a self, value: &'b T) -> Result<(), Self::Error<'a>>;
}
