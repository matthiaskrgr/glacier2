// edition:2018
// revisions: no_drop_tracking drop_tracking drop_tracking_mir
// [drop_tracking] compile-flags: -Zdrop-tracking
// [drop_tracking_mir] compile-flags: -Zdrop-tracking-mir
// [drop_tracking] check-pass
// [drop_tracking_mir] check-pass

struct Foo(*const u8);

unsafe impl Send for Foo {}

async fn overlap_never() {
    let error1 = uninhabited_async();
    noop().await;
    let error2 = uninhabited_async();
    drop(error1);
    noop().await;
    drop(error2);
}

fn assert_send<T: Send>(_: T) {}

fn main() {
    assert_send(async {
        //[no_drop_tracking]~^ ERROR future cannot be sent between threads safely
        std::io::Result::Ok(()).await;
    })
}
