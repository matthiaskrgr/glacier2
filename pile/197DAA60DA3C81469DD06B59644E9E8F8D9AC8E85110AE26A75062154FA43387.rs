// known-bug: #110395
// FIXME check-pass
// This is a non-regression test for const-qualification of unstable items in libcore
// as explained in issue #67053.
// const-qualification could miss some `const fn`s if they were unstable and the feature
// gate was not enabled in libcore.

#![stable(feature = "core", // check-pass

// Test that we can handle newtypes wrapping extern types

#![feature(extern_types)]

use std::marker::PhantomData;

extern "C" {
  pub type ExternType;
}
unsafe impl Sync for ExternType {}
static MAGIC_FFI_STATIC: u8 = 42;

#[repr(transparent)]
pub struct Wrapper(ExternType);
pub static MAGIC_FFI_REF: &'static Wrapper = unsafe {
  std::mem::transmute(&MAGIC_FFI_STATIC)
};

#[repr(transparent)]
pub struct Wrapper2(PhantomData<Vec<i32>>, ExternType);
pub static MAGIC_FFI_REF2: &'static Wrapper2 = unsafe {
  std::mem::transmute(&MAGIC_FFI_STATIC)
};

fn main() {}
 = "1.6.0")]
#![feature(staged_api, const_trait_impl)]

enum Opt<T> {
    Some(T),
    None,
}

impl<T> Opt<T> {
    #[rustc_const_unstable(feature = "foo", issue = "none")]
    #[stable(feature = "rust1", since = "1.0.0")]
    const fn unwrap_or_else<F: ~const FnOnce() -> T>(self, f: F) -> T {
    //FIXME ~^ ERROR destructor of
    //FIXME ~| ERROR destructor of
        match self {
            Opt::Some(t) => t,
            Opt::None => f(),
            //FIXME ~^ ERROR cannot call
        }
    }
}

fn main() {}
