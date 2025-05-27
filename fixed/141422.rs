#![feature(unsafe_binders)]
#![allow(incomplete_features)]

#[derive(PartialEq, Eq)]
enum O<T> {
    Some(T),
    None,
}

const C: &[O<unsafe<'a, 'b> &'b Box<dyn Fn(Box<dyn Fn() -> &'a isize>)>>] = &[O::None];

fn main() {
    let x = O::None;
    match &[x][..] {
        C => (),
        _ => (),
    }
}
