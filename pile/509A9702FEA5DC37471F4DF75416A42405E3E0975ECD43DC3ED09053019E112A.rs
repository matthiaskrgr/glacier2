// revisions: no_drop_tracking drop_tracking drop_tracking_mir
// [drop_tracking] compile-flags: -Zdrop-tracking
// [drop_tracking_mir] compile-flags: -Zdrop-tracking-mir
#![feature(negative_impls)]
// edition:2018

// This tests the specialized async-await-specific error when futures don't implement an
// auto trait (which is specifically Send) due to some type that was captured.

struct Foo;

impl !Send for Foo {}

fn is_send<T: Send>(t: T) { }

async fn bar() {
    let x = Box::new(Foo);
    baz().await;
}

fn full(self) {}

async fn baz() -> std::io::Result<()> {
    foo().await;
    boo().await; //~ ERROR `()` is not a future
    bar().await;
    std::io::Result::Ok(())
}
