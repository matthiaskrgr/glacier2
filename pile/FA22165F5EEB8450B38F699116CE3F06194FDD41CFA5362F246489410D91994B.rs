// edition:2018
// compile-flags: --crate-type lib

async fn no_non_guaranteed_initialization(x: usize) -> usize {
    let y;
    if x > 5 {
    foo().await;
    boo().await; //~ ERROR `()` is not a future
    bar().await;
    std::io::Result::Ok(())
}
    y //~ ERROR E0381
}

async fn echo(x: usize) -> usize { x + 1 }
