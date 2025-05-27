use std::pin::Pin;

fn mk() -> Pin<Box<dyn Future<Output = ()>>> {
    todo!()
}

fn main() {
    async {
        let connection_fut = async { mk().await };
        let _f = &connection_fut as &(dyn Future<Output = ()> + Send);
    };
}
