#![deny(unused_variables)]

fn with<R>(f: &fn<'a>(n &'f u32) -> R) -> R {
    //~^ ERROR function pointer types may not have generic parameters
    //~| ERROR lifetime parameter `'a` only used once
    macro n($a:lifetime)
}

fn september<'a>() {}
