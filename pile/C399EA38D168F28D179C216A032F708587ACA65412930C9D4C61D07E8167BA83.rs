use std::marker::PhantomData;

struct FsmBuilder<TFsm> {
    _fsm: PhantomData<TFsm>,
}

impl<TFsm> FsmBuilder<TFsm> {
    fn state(&mut self) -> FsmStateBuilder<TFsm> {
        todo!()
    }
}

struct FsmStateBuilder<TFsm> {
    _state: PhantomData<TFsm>,
}

impl<TFsm> FsmStateBuilder<TFsm> {
    fn on_entry<TAction: Fn(&mut StateContext<'_, TFsm>)>(&self, _action: TAction) {
    let mut builder: FsmBuilder<usize> = todo!();
    builder.state().on_entry(|_| {});
}
}

trait Fsm {
    type Context;
}

struct StateContext<'a, TFsm: Fsm> {
    context: &'a mut TFsm::Context,
}

fn main() {
    let mut builder: FsmBuilder<usize> = todo!();
    builder.state().on_entry(|_| {});
}
