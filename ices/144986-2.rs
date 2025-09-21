#![feature(async_fn_traits, unboxed_closures, explicit_tail_calls)]
#![allow(incomplete_features)]

trait Trait: AsyncFnOnce() {
    extern "rust-call" fn foo(self, _: ()) -> Self::CallOnceFuture;
}

impl<T: AsyncFnOnce()> Trait for T {
    extern "rust-call" fn foo(self, _: ()) -> T::CallOnceFuture {
        become self();
    }
}

fn main() {
    let _ = (async || {}).foo(());
}
