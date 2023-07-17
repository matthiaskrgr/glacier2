// edition:2018

async fn foo () { }
fn bar() -> impl std::future::Future { async {} }
fn boo() {}

async fn baz() -> std::io::Result<()> {
    foo().await;
    boo().await; //~ ERROR `()` is not a future
    bar().await;
    std::io::Result::Ok(())
}

macro_rules! e {
    ($x:item) => {
        ()
    };
}

macro_rules! f {
    ($expr:expr) => {
        $expr.await
        //~^ ERROR `()` is not a future
    };
}

async fn with_macros() {
    e!().await;
    //~^ ERROR `()` is not a future

    vec![123];
}

fn AtomicWaker() {}
