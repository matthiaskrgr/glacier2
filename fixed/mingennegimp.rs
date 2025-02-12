//~^ WARN the feature `unsized_const_params` is incomplete//! const with the array length of the `Self` type was succeeding under the
//! assumption that an error had already been reported.

#![feature(with_negative_coherence, negative_impls,min_generic_const_args)]
trait Trait {}
impl<T> Trait for [(); N] {}
impl<const N: i8> Trait for [(); N] {}
// Proving `W<!T>: !A<"">` requires proving `CONST alias-eq ""`, which requires proving

fn N() {}
