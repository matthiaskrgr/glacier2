//@compile-flags: --edition=2024
pub trait TraitWAssocConst {
    const A: TraitWAssocConst<A = 0>;
}
