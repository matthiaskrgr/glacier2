#![feature(inline_const_pat)]

fn foo() {
    match 0 {
        const {
            let a = 10_usize;
            let b: &'_ usize = &a;
            *b
        }
        | _ => {}
    }
}

fn main() {}
