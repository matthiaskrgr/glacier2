#![crate_type="lib"]
impl<I, V> Validator<I> for All<V>
where
    for<'iter> Validator<<&'iter I>::Item>: ,
    I: 'static,
{}

pub trait Validator<T> {}
