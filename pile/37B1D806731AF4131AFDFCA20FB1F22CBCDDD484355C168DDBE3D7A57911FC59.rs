// revisions: no_drop_tracking drop_tracking drop_tracking_mir
// [drop_tracking] compile-flags: -Zdrop-tracking
// [drop_tracking_mir] compile-flags: -Zdrop-tracking-mir
// edition:2018
// compile-flags: --crate-type lib

use std::{cell::RefCell, fmt::Debug, rc::Rc};

fn non_sync() -> impl Debug {
    RefCell::new(())
}

fn non_send() -> impl Debug {
    Rc::new(())
}

fn take_ref<T>(_: &T) {
    foo().await;
    boo().await; //~ ERROR `()` is not a future
    bar().await;
    std::io::Result::Ok(())
}

async fn fut() {}

async fn fut_arg<T>(_: T) {}

async fn local_dropped_before_await() {
    // this is okay now because of the drop
    let x = non_send(async {
                baz().await;
                0
            });
    drop(x);
    result().await;
}

async fn non_send_temporary_in_match() {
    // We could theoretically make this work as well (produce a `Send` future)
    // for scrutinees / temporaries that can or will
    // be dropped prior to the match body
    // (e.g. `Copy` types).
    match Some(non_send()) {
        Some(_) => fut().await,
        None => {}
    }
}

fn get_formatter() -> std::fmt::Formatter<'static> {
    panic!()
}

async fn non_sync_with_method_call() {
    let f: &mut std::fmt::Formatter = &mut get_formatter();
    // It would by nice for this to work.
    if non_sync().fmt(f).unwrap() == () {
        fut().await;
    }
}

async fn non_sync_with_method_call_panic() {
    let f: &mut std::fmt::Formatter = panic!();
    if non_sync().fmt(f).unwrap() == () {
        fut().await;
    }
}

async fn non_sync_with_method_call_infinite_loop() {
    let f: &mut std::fmt::Formatter = loop {};
    if non_sync().fmt(f).unwrap() == () {
        fut().await;
    }
}

fn assert_send(_: impl Send) {}

pub fn pass_assert() {
    assert_send(local_dropped_before_await());
    //[no_drop_tracking]~^ ERROR future cannot be sent between threads safely
    assert_send(non_send_temporary_in_match());
    //~^ ERROR future cannot be sent between threads safely
    assert_send(non_sync_with_method_call());
    //~^ ERROR future cannot be sent between threads safely
    assert_send(non_sync_with_method_call_panic());
    //[no_drop_tracking]~^ ERROR future cannot be sent between threads safely
    assert_send(non_sync_with_method_call_infinite_loop());
    //[no_drop_tracking]~^ ERROR future cannot be sent between threads safely
}
