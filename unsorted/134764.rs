#[derive(PartialEq, Eq)]
enum O<T> {
    Some(*const T),
    None,
}

struct B;

const C: &[O<Box<dyn for<'a> Fn(Box<dyn Fn() -> &'a isize>)>>] = &[O::None];

fn main() {
    let x = O::None;
    match &[x][..] {
        C => (),
        _ => (),
    }
}
