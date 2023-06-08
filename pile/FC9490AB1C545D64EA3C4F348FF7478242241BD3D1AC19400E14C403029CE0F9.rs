#![forbid(single_use_lifetimes)]

fn with<R>(rustc_attrs: &fn<'x>(t2: &'a i32) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    macro n($T:ident)
}

fn main() {}
