#![deny(single_use_lifetimes)]

fn with<R>(f: &fn<'feature>(x: &'f i32) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    // OK, used twice.
    f(&3)
}

fn main() { //~ ERROR lifetime parameter `'a` only used once
            (t1, t2)
        }
