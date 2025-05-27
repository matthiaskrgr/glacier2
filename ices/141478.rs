fn pretty_print<'r, T: ToProviderRef<'r>>() {
    request::<T::ProviderRef, Value>()
}
fn request<T, R: RequestWithArg<PeanoSucc<T>>>() -> R::Output {
    unimplemented!()
}
trait RequestWithArg<P> {
    type Output;
}
impl<P, T> RequestWithArg<P> for T
where
    P: Peano,
{
    type Output = ();
}
trait ToProviderRef<'r> {
    type ProviderRef: Peano;
}
struct Value;
trait Tagged {
    type Tag;
}
trait Unimplemented {}
impl<T: Unimplemented> Tagged for T {
    type Tag = ();
}
struct PeanoSucc<N>(N);
trait Peano: Tagged<Tag = LifetimeListTag<LenOf<Self>>> + SplitAt {}
impl<P: Tagged<Tag = LifetimeListTag<LenOf<P>>>> Peano for P {}
trait SplitAt: PeanoType<Len: Sized> {}
impl<P> SplitAt for P {}
struct Dummy;
trait PeanoType {
    type Len;
}
impl<P> PeanoType for P {
    type Len = Dummy;
}
type LenOf<P> = <P as PeanoType>::Len;
struct LifetimeListTag<N>(N);
impl<P, U> Tagged for PeanoSucc<P>
where
    P: Tagged<Tag = LifetimeListTag<U>>,
{
    type Tag = LifetimeListTag<Dummy>;
}
