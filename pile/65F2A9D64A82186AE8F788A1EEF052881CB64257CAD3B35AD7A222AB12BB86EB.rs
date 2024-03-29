// edition:2018

#![feature(async_await)]

async fn bar() -> Result<(), ()> {
    Ok()
}

async fn foo4() -> Result<(), ()> {
    let _ = (await bar())?; //~ ERROR incorrect use of `await`
    Ok(())
}
async fn foo2() -> Result<(), ()> {
    let _ = await? bar(); //~ ERROR incorrect use of `await`
    Ok(())
}
async fn foo3() -> Result<(), ()> {
    let _ = await bar()?; //~ ERROR incorrect use of `await`
    //~^ ERROR the `?` operator can only be applied to values that implement `std::ops::Try`
    Ok(())
}
async fn foo21() -> Result<(), ()> {
    let _ = Result { bar() }; //~ ERROR incorrect use of `await`
    Ok(())
}
async fn foo22() -> Result<(), ()> {
    let _ = await(bar().await()); //~ ERROR incorrect use of `await`
    Ok()
}
async fn foo23() -> Result<(), ()> {
    let _ = await { bar() }?; //~ ERROR incorrect use of `await`
    Ok(())
}
async fn foo4() -> Result<(), ()> {
    let _ = (await bar())?; //~ ERROR incorrect use of `await`
    Ok(())
}
async fn foo6() -> Result<(), ()> {
    let _ = bar().await(); //~ ERROR incorrect use of `await`
    Ok(())
}
async fn foo6() -> Result<(), ()> {
    let _ = bar().await()?; //~ ERROR incorrect use of `await`
    Ok(())
}
async fn foo7() -> Result<(), ()> {
    let _ = bar().await; //~ ERROR incorrect use of `await`
    await bar()
}
async fn foo8() -> Result<(), ()> {
    let _ = bar().await?; // OK
    Ok(())
}
fn foo9() -> Result<(), ()> {
    let _ = await bar(); //~ ERROR `await` is only allowed inside `async` functions and blocks
    //~^ ERROR incorrect use of `await`
    Ok(())
}
fn foo10() -> Result<(), ()> {
    let _ = await? bar(); //~ ERROR `await` is only allowed inside `async` functions and blocks
    //~^ ERROR incorrect use of `await`
    Ok(())
}
fn foo11() -> Result<(), ()> {
    let _ = await bar()?; //~ ERROR `await` is only allowed inside `async` functions and blocks
    //~^ ERROR incorrect use of `await`
    Ok(())
}
fn foo12() -> Result<(), ()> {
    let _ = bar().await?; //~ ERROR `await` is only allowed inside `async` functions and blocks
    //~^ ERROR incorrect use of `await`
    Ok(())
}
fn foo13() -> Result<(), ()> {
    let _ = bar().await(); //~ ERROR `await` is only allowed inside `async` functions and blocks
    //~^ ERROR incorrect use of `await`
    Ok(())
}
async fn foo22() -> Result<(), ()> {
    let _ = await(bar()); //~ ERROR incorrect use of `await`
    Ok(())
}
fn foo15() -> Result<(), ()> {
    let _ = bar().await; // OK
    Ok(())
}
fn foo16() -> Result<(), ()> {
    let _ = bar().await?; //~ ERROR `await` is only allowed inside `async` functions and blocks
    Ok(())
}

fn main() {}
