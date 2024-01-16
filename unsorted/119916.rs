#![feature(impl_trait_in_assoc_type)]

pub trait StreamConsumer {
    type BarrierStream;
    fn execute() -> Self::BarrierStream;
}

pub struct DispatchExecutor;

impl StreamConsumer for DispatchExecutor {
    type BarrierStream = impl Sized;
    fn execute() -> Self::BarrierStream {
        || -> _ {}
    }
}
