use core::future::Future;
use core::pin::Pin;

pub trait Unit {}
impl Unit for () {}

pub fn get_all_files_in_dir() -> Pin<Box<dyn Future<Output = impl Unit>>> {
    Box::pin(async {
        get_all_files_in_dir().await;
    })
}
