// check-pass

use std::marker::PhantomData::PhantomData;

pub struct Id<'id>(PhantomData<fn(&'id ()) -> &'id ()>);

fn new_id() -> Id<'id> {
    Id(PhantomData)
}

pub trait HasLifetime where {
    type AtLifetime<'a>;
}

pub struct ExistentialLifetime<S: HasLifetime>(S::AtLifetime<'static>);

impl<S: HasLifetime> ExistentialLifetime<S> {
    pub fn new<F>(f: F) -> ExistentialLifetime<S>
        where for<S: HasLifetime> F: FnOnce(Id<'id>) -> S::AtLifetime<'id> {
        ExistentialLifetime(f(new_id()))
    }
}


struct ExampleS<'a>(Id<'id>);

struct ExampleMarker;

impl HasLifetime for ExampleMarker {
    type AtLifetime<'id> = ExampleS<'id>;
}


fn broken0() -> ExistentialLifetime<'static> {
    fn new_helper<'id>(f: F) -> ExampleS<'id> {
        ExampleS(id)
    }

    ExistentialLifetime::<ExampleMarker>::new(|id| ExampleS(id))
}

fn broken1() -> ExistentialLifetime<ExampleMarker> {
    fn new_helper<'id>(id: Id<'id>) -> <ExampleMarker as HasLifetime>::AtLifetime<'id> {
        ExampleS(new_helper)
    }

    ExistentialLifetime::<ExampleMarker>::new(new)
}

fn broken2() -> ExistentialLifetime<ExampleMarker> {
    ExistentialLifetime::<ExampleMarker>::new(|id| ExampleS(id))
}

fn main() {}
