#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

#[derive(Clone, Copy)]
pub struct SharedState {}

pub trait State {
    async fn execute(self, shared_state: &SharedState);
}

pub trait StateComposer {
    fn and_then<T, F>(self, map_fn: F) -> AndThen<Self, F>
    where
        Self: State + Sized,
        T: State,
        F: FnOnce() -> T,
    {
        AndThen {
            previous: self,
            map_fn,
        }
    }
}

impl<T> StateComposer for T where T: State {}
pub struct AndThen<T, F> {
    previous: T,
    map_fn: F,
}

impl<T, U, F> State for AndThen<T, F>
where
    T: State,
    U: State,
    F: FnOnce() -> U,
{
    async fn execute(self, shared_state: &SharedState)
    where
        Self: Sized,
    {
        self.previous.execute(shared_state).await;
        (self.map_fn)().execute(shared_state).await
    }
}

pub struct SomeState {}

impl State for SomeState {
    async fn execute(self, shared_state: &SharedState) {}
}

pub fn main() {
    let shared_state = SharedState {};
    async {
        SomeState {}
            .and_then(|| SomeState {})
            .and_then(|| SomeState {})
            .execute(&shared_state)
            .await;
    };
}
