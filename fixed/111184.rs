use std::future::Future;

async fn data_moved_async() {}

fn run_fut<T>(fut: impl Future<Output = T>) -> T {
    loop {}
}

fn main() {
    run_fut(data_moved_async());
}
