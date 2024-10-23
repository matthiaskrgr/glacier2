trait IntFactory {
    fn stream(&self) -> impl IntFactory<stream(): Send>;
}
