struct A;
trait B<C> {
    const D: u8;
}
impl<C> B<C> for A {
    const D: u8 = 1;
}
trait E<F> {}
impl<F, G> E for G where G: B<F, D = {}> {}
impl<F> H where A: E<F> {}
