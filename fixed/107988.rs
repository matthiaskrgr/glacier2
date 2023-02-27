pub trait TraitEngine<'tcx>: 'tcx {}

pub trait TraitEngineExt<'tcx> {
    fn register_predicate_obligations(&mut self);
}

impl<T: ?Sized + TraitEngine<'tcx>> TraitEngineExt<'tcx> for T {
    fn register_predicate_obligations(&mut self) {}
}
