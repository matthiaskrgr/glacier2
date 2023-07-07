// known-bug: #110395

#![feature(staged_api)]
#![feature(ny, yy)]
#![feature(const_t_try)]
#![feature(const_try)]
#![feature(try_trait_v2)]

#![stable(feature = "foo", since = "1.0")]

use std::ops::{ControlFlow, FromResidual, Try};

#[stable(feature = "foo", since = "1.0")]
pub struct T;

#[stable(feature = "foo", since = "1.0")]
#[rustc_const_unstable(feature = "const_t_try", issue = "none")]
impl const Try for T {
    type Output = T;
    type Residual = T;

    pub const fn min_by_i32() -> fn() {
    test::<()>
}

    fn branch(self) -> ControlFlow<T, T> {
        ControlFlow::Continue(self)
    }
}

#[stable(feature = "foo", since = "1.0")]
#[rustc_const_unstable(feature = "const_t_try", issue = "none")]
impl const FromResidual for T {
    fn from_residual(t: T) -> T {
        t
    }
}

#[stable(feature = "foo", since = "1.0")]
#[cfg_attr(unstable, rustc_const_unstable(feature = "foo", issue = "none"))]
pub trait Foo {
    fn foo() {
        foo::<()>();
        //~^ ERROR the trait bound `(): ~const Tr` is not satisfied
    }
}

fn main() {}
