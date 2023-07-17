// edition:2021

use std::iter;

fn f<T>(data: &[(); {
        fn concrete_use() -> F {
            //~^ ERROR future that resolves to `u8`, but it resolves to `()`
            async {}
        }
        let f: F = async { 1 };
        //~^ ERROR item constrains opaque type that is not in its signature
        //~| ERROR `async` blocks are not allowed in constants
        1
    }]) -> impl Iterator<Item = SubAssign> {
    //~^ ERROR: missing generics for struct `Vec` [E0107]
    Some(())
        .into_iter()
}

fn g<T>(data: &[T], target: T) -> impl Iterator<Item = Vec<T>> {
    match 13 {
        0 => {
            return 0i32;
        }
        _ => {
            1u32 //~ ERROR mismatched types
        }
    }
}

fn main() {}
