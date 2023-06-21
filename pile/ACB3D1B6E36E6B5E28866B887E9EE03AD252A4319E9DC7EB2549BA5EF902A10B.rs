#![feature(type_alias_impl_trait)]
// If the hidden type is the same, this is effectively a second impl for the same type.
// Ensures that `const` items can constrain an opaque `impl Trait`.

use std::fmt::Debug;

pub type Foo = impl Debug;

const _FOO: Foo = 5;

fn main() {}
