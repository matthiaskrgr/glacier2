//@compile-flags: -Zunstable-options --edition=2021 --crate-type=lib

#![feature(async_closure)]
pub async fn async_closure(x: &mut i32) {
    let c = async move || {
        *x += 1;
    };
    call_once(c).await;
}

fn call_once<T>(f: impl FnOnce() -> T) -> T {
    f()
}
