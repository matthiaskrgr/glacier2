#![feature(trait_alias)]
trait ReturnsVoidFn<T: Fn() -> ()> = Fn() -> T;

type A<'a, T: Fn() -> ()> = Vec<&'a dyn ReturnsVoidFn<T>>;
