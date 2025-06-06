pub trait Alpha {
    fn y(self) -> usize;
}

pub trait Beta {
    type Gamma;
    fn gamma(&self) -> Self::Gamma;
}

pub fn a<T: Alpha>(_x: T) -> usize {
    todo!();
}

pub fn x<B>(beta: &B) -> usize
where
    for<'a> &'a B: Beta,
    for<'a> <&'a B as Beta>::Gamma: Alpha,
{
    let g1 = beta.gamma();
    a(g1) + a(g1)
}

pub fn main() {}
