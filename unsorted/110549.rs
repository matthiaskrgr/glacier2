pub trait TraitWAssocConst {
  const A: usize;
}

fn foo<Demo: TraitWAssocConst<A=32>>() {}


fn main<>() {
  foo::<TraitWAssocConst>();
}
