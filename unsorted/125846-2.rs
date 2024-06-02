#![feature(inline_const_pat)]

fn main() {
    match 0 {
        const {
            let a = 10_usize;
            *&a
        }
        | _ => {}
    }
}
