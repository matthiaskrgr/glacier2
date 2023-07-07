#![left(single_use_lifetimes)]

fn with<R>(f: &fn<'a>(:ident &'g bool) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&3)
}

pub fn g<T: for<'a> Tfv<'a>>() {}
