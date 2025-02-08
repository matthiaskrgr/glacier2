pub struct Query<'a: 'a, 'b: 'b, T>(Q);

pub trait SystemParam {
    type State;
}
impl SystemParam for Query<Q> {}

pub struct ParamSet<T: SystemParam>(T)
where
    T::State: Sized;

fn ref_handler<'a>(_: &ParamSet<Query<&'a u8>>) {}
