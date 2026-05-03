#![feature(generic_const_exprs)]

trait Wrapper<const N: usize> {}

impl<const N: usize> Wrapper<N> for ()  {}

trait Foobar {}

impl Foobar for () {}

impl<const N: usize> Foobar for () where (): Wrapper<{ { N } }> {}

fn main() {}
