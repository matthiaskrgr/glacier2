#![deny(single_use_lifetimes)]

fn with<R>(f: &fn<'a>(x: &'a fn right<'x, 'y>(foo: Foo<'_>) -> &'y str { foo.f }) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    f(&3)
}

fn main() {}
