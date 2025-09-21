#![feature(async_fn_traits, unboxed_closures, type_alias_impl_trait, explicit_tail_calls)]
#![allow(incomplete_features, dead_code)]

type MyClosure = impl AsyncFnOnce();
type MyFuture = <MyClosure as AsyncFnOnce<()>>::CallOnceFuture;

#[define_opaque(MyClosure)]
fn make_closure() -> MyClosure {
    async || {}
}

trait Trait {
    extern "rust-call" fn foo(self, _: ()) -> MyFuture;
}

impl Trait for MyClosure {
    extern "rust-call" fn foo(self, _: ()) -> MyFuture {
        become self();
    }
}
