pub struct Query<'a>(&'a u8);

pub trait SystemParam {
    type State;
}
impl SystemParam for Query {
    type State = ();
}

pub struct ParamSet<T: SystemParam>(T)
where
    T::State: Sized;

fn ref_handler(_: ParamSet<Query>) {}
