#![feature(async_closure)]

use std::future::Future;
use std::path::{PathBuf};

pub struct TokioOnceCell<T> {
    _a: T
}

impl<T> TokioOnceCell<T> {
    pub async fn get_or_init<F, Fut>(&self, f: F) -> &T
        where
            F: FnOnce() -> Fut,
            Fut: Future<Output = T>,
    {
        loop {}
    }
}

pub struct Struct {
    pub path: PathBuf,
    items: TokioOnceCell<Vec<()>>,
}

impl Struct {
    pub async fn get_items(&self) -> &Vec<()> {
        self.items
            .get_or_init(async move || load_items_from_path(self.path.clone()).await)
            .await
    }
}

async fn load_items_from_path(base_dir: PathBuf) -> Vec<()> {
    loop {}
}
