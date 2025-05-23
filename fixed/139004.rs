use std::{any::Any, future::Future, marker::PhantomData, pin::Pin};

pub trait Event: Send + Sync {
    type Domain: 'static;
}

pub trait EventHandler<E>: Send + Sync + 'static
where
    E: Event + Send + Sync,
{
    fn handle(self, event: &E) -> impl Future<Output = ()> + Send + '_;
}

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
type BoxedHandler = Box<dyn for<'a> Fn(&'a (dyn Any + Send + Sync + 'a)) -> BoxFuture<'a, ()> + Send + Sync>;

pub struct EventBus<D> {
    event_handlers: BoxedHandler,
    domain: PhantomData<D>,
}

impl<D> EventBus<D> {
    pub async fn subscribe<E, H>(&self, handler: H)
    where
        E: Event<Domain = D>,
        H: EventHandler<E> + Clone,
    {
        let _handler: BoxedHandler = Box::new(move |e| {
            let handler = handler.clone();
            Box::pin(async move {
                let e = e.downcast_ref::<E>().expect("downcast failed for event");
                handler.handle(e).await
            })
        });

        todo!()
    }
}
