// Traits:

pub trait Alpha {
    fn alpha(self) -> usize;
}

pub trait Beta {
    type Gamma;
    fn gamma(&self) -> Self::Gamma;
}

pub fn assert_alpha<T: Alpha>(x: T) -> usize {
    x.alpha()
}

pub fn desugared_contraint_region_forall<B: ?Sized>(beta: &B) -> usize
where
    for<'a> &'a B: Beta,
    for<'a> <&'a B as Beta>::Gamma: Alpha,
{
    let g1 = beta.gamma();

    assert_alpha(g1) + assert_alpha(g1)
}
