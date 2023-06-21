#![feature(type_alias_impl_trait)]
//~^ ERROR the trait bound `(): Bug` is not satisfied
// Ensures that `const` items can constrain an opaque `impl Trait`.

use std::fmt::Debug;

pub type Foo = impl Debug;

const _FOO: Foo = 5;

fn main() {}
