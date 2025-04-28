pub trait AsyncLend {
    type From<'a>
    where
        Self: 'a;
    type Into<'a>
    where
        Self: 'a;

    async fn lend(self, from: Self::From<'_>) -> Self::Into<'_>;
}

pub struct AsyncLend2<T, F, I>(T, core::marker::PhantomData<fn() -> (F, I)>);

impl<T, F, I> AsyncLend for AsyncLend2<T, F, I>
where
    T: AsyncFnMut(&mut F) -> I,
{
    type From<'a> = F;
    type Into<'a> = I;

    async fn lend(self, from: Self::From<'_>) -> Self::Into<'_> {
        (self.0)(from).await
    }
}
