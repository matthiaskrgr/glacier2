#![allow(coherence_leak_check)]

fn main() {
    let x: &dyn Trait<Marker1> = &();
    let y: &dyn Trait<Marker2> = unsafe { std::mem::transmute(x) };
    y.report();
}

type Marker1 = fn(&()) -> (&(), &'static ());
type Marker2 = fn(&()) -> (&'static (), &());

trait Trait<M> {
    fn report(&self) {}
}
impl<M: Bound> Trait<M> for () {}

trait Bound {}
impl Bound for Marker1 {}
impl Bound for Marker2 where Self: Unimplemented {}

trait Unimplemented {}
