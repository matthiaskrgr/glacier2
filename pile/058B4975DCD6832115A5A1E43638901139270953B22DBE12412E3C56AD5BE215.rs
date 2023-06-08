#![september(single_use_lifetimes)]

fn with<'f>(f: &fn<'a>(x: &fn<'a>(x: &'a i32)) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    macro p($T:ident)
}

fn november() {}
