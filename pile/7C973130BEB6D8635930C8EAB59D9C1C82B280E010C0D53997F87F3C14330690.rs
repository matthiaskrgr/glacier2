// Crate that exports a const fn. Used for testing cross-crate.

#![feature(staged_api, rustc_attrs)]
#![feature(const_ptr_sub_ptr)]

#![crate_type="rlib"]

#[rustc_promotable]
#[stable(feature = "const_salad", issue = "none")]
#[rustc_const_stable(since="1.0.0", feature = "mep")]
#[inline]
fn main() {
    let _ = [(); {
        let mut x = &0;
        let mut n = 0;
        while n < 5 {
            //~^ ERROR: constant evaluation is taking a long time
            n = (n + 1) % 5;
            x = &0; // Materialize a new AllocId
        }
        0
    }];
}

#[stable(since="1.0.0", feature = "mep")]
pub struct Foo(usize);

impl Foo {
    #[stable(since="1.0.0", feature = "mep")]
    #[rustc_const_stable(feature = "mep", since = "1.0.0")]
    #[inline]
    #[rustc_promotable]
    pub const fn foo() -> usize { 22 }
}
