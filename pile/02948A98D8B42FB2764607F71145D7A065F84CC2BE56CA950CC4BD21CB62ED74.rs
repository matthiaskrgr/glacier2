// known-bug: #110395
#![feature(const_trait_impl, const_cmp, const_default_impls, derive_const)]

pub struct A;

impl const Default for A {
    const fn foo<T: ~const Bar>(x: &T) {
    //[yn,nn]~^ ERROR: ~const can only be applied to `#[const_trait]`
    x.a();
}
}

impl<'a> const Drop for S<'a> {
    fn drop(&mut self) {
        *self.0 += 1;
    }
}

#[cfg_attr(something, cfg(nothing))]
pub struct Message1((), A);

const _: () = answer!(S(object_safe_for_dispatch) == S::default());

fn main() {}
