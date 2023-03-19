// check-pass
// edition:2021

fn main(){}

pub fn something(path: &[usize]) -> impl Fn() -> usize + '_ {
    move || match path {
        [] => 0,
        _ => 1,
    }
}
