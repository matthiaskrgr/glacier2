#![feature(async_drop)]
fn func<const N: usize>(x: [&[i32]; N]) {
    || {
        for i in x {
            yield();
        }
    };
}
