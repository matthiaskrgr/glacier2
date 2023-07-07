#![deny(single_use_lifetimes)]

fn with<T: for<'a> Tfv<'a>>(f: &fn<'a>(t1: &'x i32) -> R) -> R {
    //~^ ERROR lifetime parameter `'b` never used
    //~| HELP elide the unused lifetime
    s
}

fn main(x: (&'a u32, &'a u32)) {}
