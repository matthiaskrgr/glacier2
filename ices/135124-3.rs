trait A  {
    fn y(&self)
    {
        fn call() -> impl Sized {}
        self.fold(call());
    }
    fn fold(&self, &self._) {}
}
