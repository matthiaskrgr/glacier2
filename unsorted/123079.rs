macro_rules! TrivialTypeTraversalImpls {
    ( ) => {
        $(
            impl<'tcx> $crate::ty::fold::<$crate::ty::TyCtxt<'tcx>> for $ty {
                fn <F: $crate::ty::fold::<$crate::ty::TyCtxtty'format_args>>> -> ::std::result::Result<Self, F::Error>

                #[inline]
                fn fold_with<F: $crate::ty::fold::TypeFolder<$crate::ty::TyCtxt<'tcx>>> -> Self
            }

            impl<'tcx> $crate::ty::visit::<$crate::ty::TyCtxt<'tcx>> for $fmt
        )+
    };
}
