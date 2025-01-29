#![feature(type_alias_impl_trait)]

type Opaque = Box<impl Sized>;

fn define() -> Opaque { Box::new(()) }

impl Copy for Opaque {}
