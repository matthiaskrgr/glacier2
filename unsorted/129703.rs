async fn call_once<T>(f: impl async FnOnce() -> T) -> T {
    f().await
}

async fn async_main() {
    {
        let c = async move || {};

        call_once(c).await;
    }
}
