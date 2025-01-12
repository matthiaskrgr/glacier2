#![allow(coherence_leak_check)]

fn main() {
    let x: &dyn Trait<Marker1> = &();
    let y: &dyn Trait<Marker2> = unsafe { std::mem::transmute(x) };
    y.report();
}

type Marker1 = fn(&()) -> (&(), &'static ());
type Marker2 = fn(&()) -> (&'static (), &());

trait Trait<M: 'static> {
    fn report(&self);
}
impl<M: 'static> Trait<M> for () {
    fn report(&self) {
        who_am_i::<M>();
    }
}

fn who_am_i<M: 'static>() {
    let marker1 = std::any::TypeId::of::<Marker1>();
    let marker2 = std::any::TypeId::of::<Marker2>();
    let m = std::any::TypeId::of::<M>();
    let m_is = if m == marker1 {
        "Marker1"
    } else if m == marker2 {
        "Marker2"
    } else {
        unreachable!()
    };
    println!("M == {m_is}");
}
