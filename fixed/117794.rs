trait Foo {}

trait ThreeCellFragment {
    fn ext_cells<'a>(&'a self) -> impl Foo + 'a {
        self.ext_adjacent_cells(|()| 3)
    }
}
