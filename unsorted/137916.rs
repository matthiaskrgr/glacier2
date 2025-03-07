async fn make_boxed_object() -> Box<dyn Send> {
    Box::new(async {
        use std::ptr::null;
        use std::task::{Context, RawWaker, RawWakerVTable, Waker};
        let waker = unsafe {
            Waker::from_raw(RawWaker::new(
                null(),
                &RawWakerVTable::new(|_| todo!(), |_| todo!(), |_| todo!(), |_| todo!()),
            ))
        };
        let mut cx = Context::from_waker(&waker);
        let cx_ref = &mut cx;

        async {}.await;
    }) as _
}

fn main() {}
