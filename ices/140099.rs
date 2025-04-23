struct QualifiedError<E>(E);

impl<E, T> From<E> for QualifiedError<T>
where
    E:,
    for<'any> &'any mut (): Clone,
{
}

fn infallible() -> Result<(), std::convert::Infallible> {
    Ok(())
}

fn main() {
    let x = || -> Result<_, QualifiedError<_>> {
        infallible()?;
        Ok(())
    };
}
