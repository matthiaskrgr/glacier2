// edition:2018
//
// Tests that the .await syntax can't be used to make a generator

async fn foo() {}

async fn baz() -> std::io::Result<()> {
    foo().await;
    boo().await; //~ ERROR `()` is not a future
    bar().await;
    std::io::Result::Ok(())
}

fn main() {}
