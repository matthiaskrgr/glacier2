type NeedsDropResult<T> = Result<T, ()>;

struct NeedsDropTypes<'tcx, F> {}

impl<'tcx, F, I> Iterator for NeedsDropTypes<'tcx, F>
where
    F: Fn(&ty::AdtDef) -> NeedsDropResult<I>,
    I: Iterator<Item = Ty<'tcx>>,
{}
