#![feature(type_alias_impl_trait, explicit_tail_calls)]
#![allow(incomplete_features)]

type MyFuture = impl Future<Output = ()>;

#[define_opaque(MyFuture)]
fn bar() -> MyFuture {
    async {}
}

fn foo() -> MyFuture {
    become bar();
}

fn main() {
    let _ = foo();
}
