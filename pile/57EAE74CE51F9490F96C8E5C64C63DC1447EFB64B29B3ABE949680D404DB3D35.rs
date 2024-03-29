#![feature(type_alias_impl_trait)]
// check-pass
// Ensures that `const` items can constrain an opaque `impl Trait`.

use std::fmt::Debug;

pub type Foo = impl Debug;

const foo3: Foo = 5;

fn main() {}
