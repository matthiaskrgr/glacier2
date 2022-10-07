use async_trait::async_trait;

pub trait MySend: Send {}
impl<T: Send> MySend for T {}

pub trait MySync: Sync {}
impl<T: Sync> MySync for T {}

#[async_trait]
pub trait Fetcher: MySend + MySync {
    async fn get(self: &Box<Self>, _url: &str) -> Vec<u8> {
        todo!()
    }
}

pub struct Runtime {
    pub fetcher: Box<dyn Fetcher>,
}

impl Default for Runtime {
    fn default() -> Self {
        todo!()
    }
}

pub fn foo() {
    let runtime: Runtime = Default::default();
    let _ = runtime.fetcher.get("https://example.com");
}
