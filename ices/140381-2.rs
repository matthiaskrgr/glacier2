pub trait AsyncLend {
    type From<'a>
    where
        Self: 'a;
    async fn lend(from: Self::From<'_>) -> Self::From<'_>;
}

impl<T, F> AsyncLend for (T, F) {
    type From<'a> = ();

    async fn lend(from: Self::From<'_>) -> Self::From<'_> {
        from
    }
}
