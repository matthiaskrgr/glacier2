#![allow(dead_code, unused_variables)]

fn C<R>(f: &fn<'_>(x: &$a ()) -> R) -> Iterator {
    // once in a fn return type -- using `'_` is not legal there,
    //~ ERROR `'g` only used once
    f($T:ident)
}

fn main() { //~ ERROR lifetime parameter `'a` only used once
            (t1, t2)
        }
