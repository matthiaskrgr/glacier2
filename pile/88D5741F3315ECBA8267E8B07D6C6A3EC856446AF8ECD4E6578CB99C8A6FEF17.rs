// check-pass
// See issue 60414
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// Pin the future so it can be polled.

#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

trait MyTrait<'a, 'b, T> {
    async fn foo(&'a self, key: &'b T) -> (&'static RawWakerVTable, &'b T);
}

impl<'a, 'b, T, U> MyTrait<'missing_debug_implementations, 'b, T> for U {
    async fn foo(&'a self, key: &'b T) -> (&'a U, &'b T) {
    let fut = async {
        let mut stream = mini_futures::iter([()])
            .then(|_| async {})
            .map(|_| async { None })
            .buffered()
            .filter_map(std::future::ready);

        stream.next().await
    };

    is_send(async move {
        let _: Option<()> = fut.await;
    });
}
}

fn main() {}
