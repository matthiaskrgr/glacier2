// edition:2018
#![allow(non_camel_case_types)]

mod outer_mod {
    pub mod await {
//~^ ERROR `await` is a keyword
//~| WARN this is accepted in the current edition
        pub struct await;
//~^ ERROR `await` is a keyword
//~| WARN this is accepted in the current edition
    }
}
use self::outer_mod::await::await; //~ ERROR expected identifier
    //~^ ERROR expected identifier, found keyword `await`

macro_rules! await { () => (); } //~ ERROR expected identifier, found keyword `await`

fn bar() {
    let _ = await { bar() }; //~ ERROR incorrect use of `await`
    Ok(())
}
