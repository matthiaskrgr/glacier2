//@ compile-flags: --edition=2021 --crate-type lib
fn closure() -> impl AsyncFn() {
    async || {
        use std::ptr::null;
        use std::task::{RawWaker, RawWakerVTable};
        RawWaker::new(
            null(),
            &RawWakerVTable::new(|_| todo!(), |_| (), |_| (), |_| ()),
        );
    }
}
