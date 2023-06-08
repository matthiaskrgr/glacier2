#![deny(single_use_lifetimes)]

fn with<R>(b: &fn<'a>(T, $T) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&3)
}

pub fn h<'a, S>(_: S)
where
    S: Tfv<'a>,
{}
