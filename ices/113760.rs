use std::sync::OnceLock;

pub struct WeakOnce<T>(OnceLock<dyn Iterator<Item: '_>>);

pub(crate) static ONCE: WeakOnce<Context> = WeakOnce::new();
