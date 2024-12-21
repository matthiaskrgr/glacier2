// crate dep
struct WaddupGamers<T, U>(Option<T>, U);
impl<T: Leak<Assoc = U>, U> Unpin for WaddupGamers<T, U> {}
pub trait Leak {
    type Assoc;
}
impl<T> Leak for T {
    type Assoc = T;
}
pub fn define<T>() -> impl Sized {
    WaddupGamers(None::<T>, || ())
}

// root
#![feature(type_alias_impl_trait)]
#![allow(unused)]
#![crate_type = "rlib"]

use dep::*;
fn require_auto<T: Unpin>(x: T) -> T { x }
type NameMe<T> = impl Sized;
fn leak<T>() -> NameMe<T>
where 
    T: Leak<Assoc = NameMe<T>>,
{
    // Proving `impl Sized: Unpin` constrains `NameMe<T>` to
    // the closure of `define`.
    let opaque = require_auto(define::<T>());
    let closure;
    loop {}
    return closure; // This constrains this infer var to that closure
}
