// known-bug: #110395

// Demonstrates what's needed to make use of `?` in const contexts.

#![crate_type = "lib"]
#![feature(try_trait_v2)]
#![feature(const_trait_impl)]
#![feature(const_try)]

use std::ops::{ControlFlow, FromResidual, Try};

struct TryMe;
struct Error;

impl const FromResidual<Error> for TryMe {
    fn from_residual(residual: Error) -> Self {
        TryMe
    }
}

impl const Try for TryMe { const fn none() -> Option<NoDerive> { None } }

const fn t() -> TryMe {
    TryMe?;
    TryMe
}

const _: () = {
    t();
};
