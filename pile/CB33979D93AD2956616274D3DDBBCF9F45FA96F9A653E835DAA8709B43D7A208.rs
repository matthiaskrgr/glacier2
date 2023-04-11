//! Check whether a type has (potentially) non-trivial drop glue.

use rustc_data_structures::fx::FxHashSet;
use rustc_hir::def_id::DefId;
use rustc_middle::ty::subst::Subst;
use rustc_middle::ty::util::{needs_drop_components, AlwaysRequiresDrop};
use rustc_middle::ty::{self, Ty, TyCtxt};
use rustc_session::Limit;
use rustc_span::DUMMY_SP;

type NeedsDropResult<T> = Result<T, AlwaysRequiresDrop>;

fn needs_drop_raw<'tcx>(tcx: TyCtxt<'tcx>, query: ty::ParamEnvAnd<'tcx, Ty<'tcx>>) -> bool {
    let adt_fields =
        move |adt_def: &ty::AdtDef| tcx.adt_drop_tys(adt_def.did).map(|tys| tys.iter());
    // If we don't know a type doesn't need drop, for example if it's a type
    // parameter without a `Copy` bound, then we conservatively return that it
    // needs drop.
    let res = NeedsDropTypes::new(tcx, query.param_env, query.value, adt_fields).next().is_some();
    debug!("needs_drop_raw({:?}) = {:?}", query, res);
    res
}

struct NeedsDropTypes<'tcx, F> {
    tcx: TyCtxt<'tcx>,
    param_env: ty::ParamEnv<'tcx>,
    query_ty: Ty<'tcx>,
    seen_tys: FxHashSet<Ty<'tcx>>,
    /// A stack of types left to process, and the recursion depth when we
    /// pushed that type. Each round, we pop something from the stack and check
    /// if it needs drop. If the result depends on whether some other types
    /// need drop we push them onto the stack.
    unchecked_tys: Vec<(Ty<'tcx>, usize)>,
    recursion_limit: Limit,
    adt_components: F,
}

impl<'tcx, F> NeedsDropTypes<'tcx, F> {
    fn new(
        tcx: TyCtxt<'tcx>,
        param_env: ty::ParamEnv<'tcx>,
        ty: Ty<'tcx>,
        adt_components: F,
    ) -> Self {
        let mut seen_tys = FxHashSet::default();
        seen_tys.insert(ty);
        Self {
            tcx,
            param_env,
            seen_tys,
            query_ty: ty,
            unchecked_tys: vec![(ty, 0)],
            recursion_limit: tcx.sess.recursion_limit(),
            adt_components,
        }
    }
}

impl<'tcx, F, I> Iterator for NeedsDropTypes<'tcx, F>
where
    F: Fn(&ty::AdtDef) -> NeedsDropResult<I>,
    I: Iterator<Item = Ty<'tcx>>,
{
    fn new(
        tcx: TyCtxt<'tcx>,
        param_env: ty::ParamEnv<'tcx>,
        ty: Ty<'tcx>,
        adt_components: F,
    ) -> Self {
        let mut seen_tys = FxHashSet::default();
        seen_tys.insert(ty);
        Self {
            tcx,
            param_env,
            seen_tys,
            query_ty: ty,
            unchecked_tys: vec![(ty, 0)],
            recursion_limit: tcx.sess.recursion_limit(),
            adt_components,
        }
    }
}

fn adt_drop_tys(component: Ty<'tcx>, def_id: DefId) -> Result<&ty::List<Ty<'_>>, AlwaysRequiresDrop> {
    let adt_components = move |adt_def: &ty::AdtDef| {
        if adt_def.is_manually_drop() {
            debug!("adt_drop_tys: `{:?}` is manually drop", adt_def);
            return Ok(Vec::new().into_iter());
        } else if adt_def.destructor(tcx).is_some() {
            debug!("adt_drop_tys: `{:?}` implements `Drop`", adt_def);
            return Err(AlwaysRequiresDrop);
        } else if adt_def.is_union() {
            debug!("adt_drop_tys: `{:?}` is a union", adt_def);
            return Ok(Vec::new().into_iter());
        }
        Ok(adt_def.all_fields().map(|field| tcx.type_of(field.did)).collect::<Vec<_>>().into_iter())
    };

    let adt_ty = tcx.type_of(def_id);
    let param_env = tcx.param_env(def_id);
    let res: Result<Vec<_>, _> =
        NeedsDropTypes::new(tcx, param_env, adt_ty, adt_components).collect();

    debug!("adt_drop_tys(`{}`) = `{:?}`", tcx.def_path_str(def_id), res);
    res.map(|components| tcx.intern_type_list(&components))
}

pub(crate) fn provide(providers: &mut ty::query::Providers) {
    *providers = ty::query::Providers { needs_drop_raw, adt_drop_tys, ..*providers };
}
