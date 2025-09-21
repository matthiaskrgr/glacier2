//@compile-flags: --edition=2024
fn create_complex_future() -> impl Future<Output = impl ReturnsSend> {
    async { create_complex_future().await }
}

trait ReturnsSend {}
