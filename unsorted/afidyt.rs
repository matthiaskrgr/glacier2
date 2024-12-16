//@compile-flags: --edition=2021
#![feature(async_fn_in_dyn_trait)]

trait AsyncTrait {
    async fn async_dispatch(self: AsyncTrait) -> Self::Output;
}
