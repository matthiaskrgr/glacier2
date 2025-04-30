//@compile-flags: --edition=2024
#![feature(async_drop)]
async fn move_part_await_return_rest_tuple() -> Vec<usize> {
    let x = (vec![3], vec![4, 4]);
    drop(x.1);

    x.0
}
