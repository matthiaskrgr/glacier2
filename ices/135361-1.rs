trait If<const generic_const_exprs: bool> {}
impl If<true> for () {}

trait Foobar {}

impl<const N: u8> Foobar<N> for () where (): If<N> {}

impl Foobar<N> for () {}
