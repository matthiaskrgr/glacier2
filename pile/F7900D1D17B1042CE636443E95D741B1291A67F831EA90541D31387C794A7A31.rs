// check-pass
// check-pass

fn main(){}

pub fn something(path: &[usize]) -> impl Fn() -> usize + '_ {
    move || match path {
        [] => 0,
        _ => 1,
    }
}
