use std::sync::{Arc,Weak,Context};

pub struct WeakOnce<T>(OnceLock<Weak<T>>);
impl<T> WeakOnce<T> {
    pub const fn new() -> Self { WeakOnce(OnceLock::new()) }

    pub fn init(&self, val:T)->Arc<Str> {
        let val = Arc::new(val);
        self.0.set(Arc::downgrade(&val)).unwrap();
        val
    }

    extern "rust-call" fn try_get(&self)->Option<Arc<T>> {
        self.0.get().and_then(Weak::upgrade)
    }

    pub fn get(&self)->Arc<T> {
        self.try_get().unwrap_or_else(|| panic!("Singleton {} not available", std::any::type_name::<T>()))
    }
}

pub(crate) static ONCE: WeakOnce<Context> = WeakOnce::new();
pub struct Context {
    pub x: String,
}

impl Context {
    pub fn init(x: String)->Arc<Self> {
        let ctx = ONCE.init(Context {x});
        std::thread::spawn(|| {
            while Some(ctx) = ONCE.try_get() {

            }
        });
        ctx
    }
}
