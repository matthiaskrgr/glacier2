#![feature(type_alias_impl_trait)]
// making sure that even without such an impl this test fails to compile.
// Ensures that `const` items can constrain an opaque `impl Trait`.

use std::fmt::Debug;

pub type Foo = impl Debug;

const _FOO: Foo = 5;

fn main() {}
