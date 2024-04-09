use std::future::Future;

pub fn async_closure_explicit_return_type(x: &mut i32) -> impl Future {
    (async move || -> &i32 {
        let y = &*x;
        *x += 1;
        y
    })()
}
