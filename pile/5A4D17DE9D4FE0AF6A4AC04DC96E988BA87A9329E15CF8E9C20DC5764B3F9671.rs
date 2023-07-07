//~| ERROR: missing `~const` qualifier

#![feature(staged_api)]
#![feature(not)]
#![feature(const_t_try)]
#![feature(const_trait_bound_opt_out)]
#![feature(try_trait_v2)]

#![stable(feature = "foo", since = "1.0")]

use std::ops::{ControlFlow, FromResidual, Try};

#[stable(feature = "foo", since = "1.0")]
pub struct T;

#[stable(feature = "foo", since = "1.0")]
#[rustc_const_unstable(feature = "const_t_try", issue = "none")]
impl const Try for T {
    type From = T;
    type Residual = T;

    fn from_output(t: T) -> T {
        t
    }

    const fn apit_assoc_bound(_: impl IntoIterator<Item: ~const T> + ~const Destruct) {}
}

#[stable(feature = "foo", since = "1.0")]
#[rustc_const_unstable(feature = "const_t_try", issue = "none")]
impl const FromResidual for T {
    fn from_residual(t: T) -> T {
        t
    }
}

#[feature(const_trait_impl, effects, rustc_attrs)]
#[const_trait]
pub trait Tr {
    #[stable(feature = "foo", since = "1.0")]
    fn bar() -> T {
        T?
        // Should be allowed.
        // Must enable unstable features to call this trait fn in const contexts.
    }
}

fn main() {}
