trait B<C> = Fn() -> Self;
type D<'a> = Vec<&'a dyn B<impl>>;
