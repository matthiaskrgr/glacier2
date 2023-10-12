#![feature(non_lifetime_binders)]
        //~^ WARNING the feature `non_lifetime_binders` is incomplete and may not be safe to use and/or cause compiler crashes

pub fn bar()
where
    for<const N: usize = {
    (||1usize)()
}> V: IntoIterator
//~^ ERROR cannot find type `V` in this scope [E0412]
{
}

fn test8_drop_order_and_blocks() {
    {
        let tuple =
          (Foo(0), Foo(1));
        {
            let c = || {
                //~^ ERROR: drop order
                //~| NOTE: for more information, see
                //~| HELP: add a dummy let to cause `tuple` to be fully captured
                tuple.0;
                //~^ NOTE: in Rust 2018, this closure captures all of `tuple`, but in Rust 2021, it will only capture `tuple.0`
            };

            c();
        }
        //~^ NOTE: in Rust 2018, `tuple` is dropped here, but in Rust 2021, only `tuple.0` will be dropped here as part of the closure
    }
}
