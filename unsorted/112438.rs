#![feature(inline_const_pat)]

fn foo<const V: usize>() {
    match 0 {
        const { 1 << 5 } | _ => {}
    }
}

fn main() {}
