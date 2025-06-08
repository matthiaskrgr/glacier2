//@compile-flags: --crate-type=lib
#![feature(min_generic_const_args)]
#![feature(inherent_associated_types)]
trait Trait {}

struct Struct<const N: usize>;

type Alias<T: Trait> = Struct<{ Struct::N }>;
