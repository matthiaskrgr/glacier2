//@ check-pass
//@ edition: 2021
// Precise capture struct by move, consume field

struct Vec { a: i32 }

pub async fn async_closure(x: &mut i32) {
    let c = async move || {
        *x += 1;
    };
    call_once(c).await;
}

fn call_once<T>(f: impl FnOnce() -> T) -> T {
    f()
}
