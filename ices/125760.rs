#![feature(type_alias_impl_trait)]
type Bar = impl std::fmt::Display;

async fn test<const N: crate::Bar>() {}

fn main () {}
