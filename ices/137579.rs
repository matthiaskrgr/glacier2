#![allow(incomplete_features)]
#![feature(dyn_star)]

trait Trait {}
impl Trait for () {}

struct Wrapper<T: ?Sized>(T);

const OBJECT: *const (dyn Trait + Send) = &();
const _: *const Wrapper<dyn* Drop> = OBJECT as _;

fn main() {}
