#![feature(return_position_impl_trait_in_trait)]

use std::future::Future;

pub trait AsyncTrait {
    fn async_fn_multiple<'a>(&'a self, buff: &[u8]) -> impl Future<Output = Wrapper<impl Sized>>;
}

pub struct Struct;

impl AsyncTrait for Struct {
    fn async_fn_multiple<'a, 'b>(&'a self, buff: &'b [u8]) -> impl Future {}
}
