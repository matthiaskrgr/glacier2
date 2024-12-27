//@compile-flags: --edition=2024
trait Owned: 'static {}
fn ice() -> impl Future<Output = &'static dyn Owned> {
    async {
        let not_static = 0;
        force_send(async_load(&not_static));
        loop {}
    }
}

fn force_send<T: Send>(_: T) {}

fn async_load<'a, T: LoadQuery<'a>>(this: T) -> impl Future {
    async {
        this.get_future().await;
    }
}

trait LoadQuery<'a>: Sized {
    type LoadFuture: Future;

    fn get_future(self) -> Self::LoadFuture {
        loop {}
    }
}

impl<'a> LoadQuery<'a> for &'a u8 {
    type LoadFuture = ();
}
